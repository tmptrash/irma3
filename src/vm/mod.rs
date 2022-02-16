//!
//! Virtual Machine module. Implements all atom types and related stuff. Should be
//! optimized for speed. There are many virtual machines in a world at the same time.
//! One VM runs one molecule.
//! 
pub mod buf;
pub mod vmdata;
pub mod ret;

use share::global::Atom;
use share::global::{*};
use share::atom;
use atom::{*};
use ret::Return;
use vmdata::VMData;
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOMS_MAP: &[fn(&mut VM, Atom, &mut VMData) -> Return] = &[
    VM::atom_empty,  // 0 - must be an empty fn. Means empty cell or no atom
    VM::atom_mov,
    VM::atom_fix,
    VM::atom_spl,
    VM::atom_if,
    VM::atom_job,
    VM::atom_empty,  // unused
    VM::atom_empty   // unused
];
///
/// Index of if atom. Must be synchronized with ATOMS_MAP
///
const ATOM_IF: Atom = 4;
///
/// Describes data for one instance of Virtual Machine
///
#[derive(Copy, Clone)]
pub struct VM {
    ///
    /// Energy of current VM. Every VM may have it's own.
    ///
    energy: isize,
    ///
    /// Offset of current atom, which VM in running.
    ///
    offs: Offs
}

impl VM {
    pub fn new(energy: isize, offs: Offs) -> VM {
        VM {
            energy,
            offs
        }
    }
    ///
    /// Runs one atom depending on type and moves VM to the next one depending on
    /// atom direction.
    ///
    pub fn run_atom(&mut self, vm_data: &mut VMData) -> Return {
        let atom: Atom = vm_data.world.get_atom(self.offs);
        let atom_type = get_type(atom);
        if atom_type == ATOM_EMPTY { return Return::Code(RET_SKIPPED) }

        ATOMS_MAP[atom_type as I](self, atom, vm_data)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms together.
    /// Should be optimized by speed. After moving all bonds should not be broken.
    ///
    pub fn atom_mov(&mut self, mut atom: Atom, vm_data: &mut VMData) -> Return {
        let mut offs: Offs;
        let mut to_offs: Offs;
        let mut d0: Dir;
        let mut d1: Dir;
        let mut a: Atom;
        let mut o: Offs;
        let dir   = (atom & ATOM_MOV_DIR >> ATOM_MOV_DIR_SHIFT) as Dir;   // atom move direction
        let stack = &mut vm_data.buf.stack;
        let wrld  = &mut vm_data.world;
        let buf   = &mut vm_data.buf.buf;
        let mov_energy = vm_data.atoms_cfg.mov_energy;

        stack.clear();                                                    // every call of mov should reset stack & buf
        buf.clear();
        stack.push(self.offs);

        while !stack.empty() {                                            // before while, stack should have >= 1 atoms
            offs = stack.last().unwrap();                                 // offset of atom before move
            atom = wrld.get_atom(offs);                                   // atom we have to move
            to_offs = wrld.get_offs(offs, dir);                           // destination atom position
            if is_atom(wrld.get_atom(to_offs)) {                          // can't move atom. Another one is there
                stack.push(to_offs);
                continue;
            }
            stack.shrink();                                               // destination cell is empty, can move there
            wrld.mov_atom(offs, to_offs, atom, vm_data.io); // move atom physically
            buf.insert(to_offs);                                          // mark atom as "already moved"
            self.energy -= mov_energy;                                    // decrease energy for every moved atom
            // update vm bond of moved atom---------------------------------------------------------------------------------
            d0 = get_vm_dir(atom);                                        // get VM dir of moved atom
            d1 = DIR_MOV_ATOM[d0 as I][dir as I];                         // final dir of moved atom
            o  = wrld.get_offs(offs, d0);                             // offs of near atom
            if d1 == DIR_NO { buf.insert(o); }                            // near atom is to far, will add it later
            else {
                set_vm_dir(&mut atom, d1);                            // distance between atoms is 1. update bond
                set_vm_bond(&mut atom);
                wrld.set_atom(to_offs, atom, vm_data.io);
                // update vm bond of near atom------------------------------------------------------------------------------
                d0 = DIR_REV[d0 as I];                                    // get near atom's dir to moved atom
                a  = wrld.get_atom(o);                               // near atom
                if get_vm_dir(a) == d0 {                             // near atom has a bond with moved
                    d1 = DIR_NEAR_ATOM[d0 as I][dir as I];                // final dir of near atom
                    set_vm_dir(&mut a, d1);
                    set_vm_bond(&mut a);
                    wrld.set_atom(o, a, vm_data.io);
                }
            }

            if get_type(atom) == ATOM_IF {                                // if atom has additional then bond
                // update then bond of moved atom---------------------------------------------------------------------------
                d0 = get_dir2(atom);                                      // get then dir of if moved atom
                d1 = DIR_MOV_ATOM[d0 as I][dir as I];                     // final dir of if moved atom
                o  = wrld.get_offs(offs, d0);                         // offs of near atom
                if d1 == DIR_NO { buf.insert(o); }                        // near atom is to far, will add it later
                else {
                    set_dir2(&mut atom, d1);                          // distance between atoms is 1. update bond
                    wrld.set_atom(to_offs, atom, vm_data.io);
                    // update then bond of near atom------------------------------------------------------------------------
                    d0 = DIR_REV[d0 as I];                                // get near atom's dir to moved atom
                    a  = wrld.get_atom(o);                           // near atom
                    if get_dir2(a) == d0 {                           // near atom has a bond with moved
                        d1 = DIR_NEAR_ATOM[d0 as I][dir as I];            // final dir of near atom
                        set_dir2(&mut a, d1);
                        wrld.set_atom(o, a, vm_data.io);
                    }
                }
            }
        }
        if has_vm_bond(atom) {
            self.offs = wrld.get_offs(self.offs, get_vm_dir(atom));
        }

        Return::Code(RET_OK)
    }
    ///
    /// Implements fix atom. Creates vm bond between two atoms. If vm bond is already exist, than
    /// try to create if/then bond for if atom. Consumes energy.
    ///
    pub fn atom_fix(&mut self, atom: Atom, vm_data: &mut  VMData) -> Return {
        let offs0 = vm_data.world.get_offs(self.offs, get_dir1(atom));       // gets first near atom offs to fix
        let mut atom0 = vm_data.world.get_atom(offs0);                       // gets first near atom to fix
        if !is_atom(atom0) { return Return::Code(RET_SKIPPED) }              // no first near atom to fix
        let d0 = get_dir2(atom);
        if !is_atom(vm_data.world.get_dir_atom(offs0, d0)) { return Return::Code(RET_SKIPPED) }  // there is no second near atom to fix

        // fix vm bond------------------------------------------------------------------------------------------------------
        if !has_vm_bond(atom0) {                                             // first near atom has no vm bond
            set_vm_dir(&mut atom0, d0);
            set_vm_bond(&mut atom0);
            vm_data.world.set_atom(offs0, atom0, vm_data.io);
            if has_vm_bond(atom) { self.offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy -= vm_data.atoms_cfg.fix_energy;
            return Return::Code(RET_OK);
        }
        if get_type(atom0) != ATOM_IF { return Return::Code(RET_SKIPPED) }   // only if atom has if and then bonds
        // fix then bond----------------------------------------------------------------------------------------------------
        if !has_dir2_bond(atom0) {                                           // first near atom has no then bond
            set_dir2(&mut atom0, d0);
            set_dir2_bond(&mut atom0);
            vm_data.world.set_atom(offs0, atom0, vm_data.io);
            if has_vm_bond(atom) { self.offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy -= vm_data.atoms_cfg.fix_energy;
            return Return::Code(RET_OK);
        }

        Return::Code(RET_SKIPPED)
    }
    ///
    /// Implements spl atom. Splits two atoms bonds. If atoms has no vm bond, than
    /// try to split if/then bonds for if atom. Releases energy.
    ///
    pub fn atom_spl(&mut self, atom: Atom, vm_data: &mut  VMData) -> Return {
        let offs0 = vm_data.world.get_offs(self.offs, get_dir1(atom));       // gets first near atom offs to split
        let mut atom0 = vm_data.world.get_atom(offs0);                       // gets first near atom to split
        if !is_atom(atom0) { return Return::Code(RET_SKIPPED) }              // no first near atom to split
        let d0 = get_dir2(atom);
        if !is_atom(vm_data.world.get_dir_atom(offs0, d0)) { return Return::Code(RET_SKIPPED) }  // there is no second near atom to split

        // split vm bond----------------------------------------------------------------------------------------------------
        if has_vm_bond(atom0) {                                              // first near atom has vm bond
            reset_vm_bond(&mut atom0);
            vm_data.world.set_atom(offs0, atom0, vm_data.io);
            if has_vm_bond(atom) { self.offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy += vm_data.atoms_cfg.spl_energy;
            return Return::Code(RET_OK);
        }
        if get_type(atom0) != ATOM_IF { return Return::Code(RET_SKIPPED) }
        // split then bond--------------------------------------------------------------------------------------------------
        if has_dir2_bond(atom0) {                                            // first near atom has then bond
            reset_dir2_bond(&mut atom0);
            vm_data.world.set_atom(offs0, atom0, vm_data.io);
            if has_vm_bond(atom) { self.offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy += vm_data.atoms_cfg.spl_energy;
            return Return::Code(RET_OK);
        }

        Return::Code(RET_SKIPPED)
    }
    ///
    /// Implements cond command. Depending on the condition VM will run one of two
    /// possible atoms.
    ///
    pub fn atom_if(&mut self, atom: Atom, vm_data: &mut VMData) -> Return {
        // runs if -> then scenario
        if has_dir2_bond(atom) && is_atom(vm_data.world.get_dir_atom(self.offs, get_dir1(atom))) {
            self.offs = vm_data.world.get_offs(self.offs, get_dir2(atom));
            self.energy -= vm_data.atoms_cfg.if_energy;
            return Return::Code(RET_OK);
        }
        // runs else scenario
        if has_vm_bond(atom) {
            self.offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom));
            self.energy -= vm_data.atoms_cfg.if_energy;
            return Return::Code(RET_OK);
        }

        Return::Code(RET_SKIPPED)
    }
    ///
    /// Implements job command. Creates one new VM instance (thread). Energy decreasing
    /// should be called from outside, because new VM is added there
    ///
    pub fn atom_job(&mut self, atom: Atom, vm_data: &mut VMData) -> Return {
        let offs = vm_data.world.get_offs(self.offs, get_vm_dir(atom));
        if !is_atom(vm_data.world.get_atom(offs)) { return Return::Code(RET_SKIPPED) }
        //self.energy -= energy;  // should be outside of VM
        Return::AddVm(self.energy / 2, offs)
    }
    ///
    /// Return energy amount of current VM
    ///
    pub fn get_energy(&self) -> isize { self.energy }
    ///
    /// Decreases energy amount for current VM
    ///
    pub fn dec_energy(&mut self, energy: isize) { self.energy -= energy; }
    ///
    /// Just a stub for empty atom in a world
    ///
    fn atom_empty(&mut self, _atom: Atom, _vm_data: &mut VMData) -> Return { Return::Code(RET_SKIPPED) }
}