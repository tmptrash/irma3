//!
//! Virtual Machine module. Implements all atom types and related stuff. Should be
//! optimized for speed. There are many virtual machines in a world at the same time.
//! One VM runs one molecule.
//! 
pub mod buf;
pub mod atom;

use std::convert::TryInto;
use log::{*};
use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;
use atom::{*};
use crate::global::{*};
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOMS_MAP: &'static [fn(&mut VM, Atom, &mut VMData) -> bool] = &[
    VM::atom_empty,  // must be an empty fn. Means empty cell or no atom
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
pub struct VM {
    ///
    /// Energy of current VM. Every VM may have it's own.
    ///
    energy: usize,
    ///
    /// Offset of current atom, which VM in running.
    ///
    offs: Offs
}
///
/// Data needed for VM to work. Should be set from outside of VM
///
pub struct VMData {
    ///
    /// Reference to the world data
    ///
    pub world: World,
    ///
    /// Shared between VMs buffer. Is used in mov atom.
    ///
    pub buf: MoveBuffer,
    ///
    /// Reverted directions, which is used in mov atom
    ///
    pub dirs_rev: [Dir; DIRS_LEN]
}

impl VM {
    pub fn new() -> VM {
        VM {
            energy: 0,
            offs: 0
        }
    }
    ///
    /// Creates a list of VMs. It's size depends on configuration.
    ///
    pub fn create_vms(vm_amount: usize) -> Vec<VM> {
        let mut v: Vec<VM> = Vec::with_capacity(vm_amount); 
        for _i in 0..vm_amount {
            v.push(VM::new());
        }
        v
    }
    ///
    /// Runs one atom depending on type and moves VM to the next one depending on
    /// atom direction.
    ///
    pub fn run_atom(&mut self, vm_data: &mut VMData) -> bool {
        let atom: Atom = vm_data.world.get_dot(self.offs);
        let atom_type = get_type(atom);
        if atom_type == ATOM_EMPTY { return false }

        let dir = get_vm_dir(atom);
        if dir >= vm_data.world.dirs.len() as Dir {
            warn!("Invalid direction. Offs: {}, Atom: {}, Dir: {}", self.offs, atom, dir);
            return false;
        }
        self.offs = vm_data.world.get_offs(self.offs, dir);

        ATOMS_MAP[atom_type as I](self, atom, vm_data)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms together.
    /// Should be optimized by speed. After moving all bonds should not be broken.
    ///
    pub fn atom_mov(&mut self, mut atom: Atom, vm_data: &mut VMData) -> bool {
        let mut offs: Offs;
        let mut to_offs: Offs;
        let mut d0: Dir;
        let mut d1: Dir;
        let mut a: Atom;
        let mut o: Offs;
        let dir = (atom & ATOM_MOV_DIR >> ATOM_MOV_DIR_SHIFT) as Dir;     // atom move direction
        let stack = &mut vm_data.buf.stack;
        let wrld  = &mut vm_data.world;
        let buf   = &mut vm_data.buf.buf;

        stack.clear();                                                    // every call of mov should reset stack & buf
        buf.clear();
        stack.push(self.offs);

        while stack.not_empty() {                                         // before while, stack should have >= 1 atoms
            offs = stack.last();                                          // offset of atom before move
            atom = wrld.get_dot(offs);                                    // atom we have to move
            to_offs = wrld.get_offs(offs, dir);                           // destination atom position
            if is_atom(wrld.get_dot(to_offs)) {                           // can't move atom. Another one is there
                stack.push(to_offs);
                continue;
            }
            stack.shrink();                                               // destination cell is empty, can move there
            wrld.mov_dot(offs, to_offs, atom);                            // move atom physically
            buf.insert(to_offs);                                          // mark atom as "already moved"
            // update vm bond of moved atom---------------------------------------------------------------------------------
            d0 = get_vm_dir(atom);                                        // get VM dir of moved atom
            d1 = DIR_MOV_ATOM[d0 as I][dir as I];                         // final dir of moved atom
            o = wrld.get_offs(offs, d0);                                  // offs of near atom
            if d1 == DIR_NO { buf.insert(o); }                            // near atom is to far, will add it later
            else {
                set_vm_dir(atom, d1);                                     // distance between atoms is 1. update bond
                wrld.set_dot(to_offs, atom);
                // update vm bond of near atom------------------------------------------------------------------------------
                d0 = DIR_REV[d0 as I];                                    // get near atom's dir to moved atom
                a = wrld.get_dot(o);                                      // near atom
                if get_vm_dir(a) == d0 {                                  // near atom has a bond with moved
                    d1 = DIR_NEAR_ATOM[d0 as I][dir as I];                // final dir of near atom
                    set_vm_dir(a, d1);
                    wrld.set_dot(o, a);
                }
            }
            // TODO: start refactoring from here
            if get_type(atom) == ATOM_IF {                                // if atom has additional else and then bonds
                // update bonds of moved atom-------------------------------------------------------------------------------
                d0 = get_if_dir(atom);                                    // get if dir of moved atom
                d1 = DIR_MOV_ATOM[d0 as I][dir as I];                     // final dir of if moved atom
                if d1 == DIR_NO { buf.insert(wrld.get_offs(offs, d0)); }  // near atom is to far, will add it later
                else { set_if_dir(atom, d1); }                            // distance between atoms is 1. update bond
                // update bonds of near atom--------------------------------------------------------------------------------
                a = wrld.get_dot(wrld.get_offs(offs, d0));                // get near atom
                d0 = DIR_REV[d0 as I];                                    // get near atom's dir to moved atom
                d1 = DIR_NEAR_ATOM[d0 as I][dir as I];                    // final dir of near atom
                if d1 != DIR_NO { set_if_dir(a, d1); }                    // distance between atoms is 1. update bond
                // update bonds of moved atom-------------------------------------------------------------------------------
                d0 = get_then_dir(atom);                                  // get if dir of moved atom
                d1 = DIR_MOV_ATOM[d0 as I][dir as I];                     // final dir of if moved atom
                if d1 == DIR_NO { buf.insert(wrld.get_offs(offs, d0)); }  // near atom is to far, will add it later
                else { set_then_dir(atom, d1); }                          // distance between atoms is 1. update bond
                // update bonds of near atom--------------------------------------------------------------------------------
                d0 = DIR_REV[d0 as I];                                    // get near atom's dir to moved atom
                d1 = DIR_NEAR_ATOM[d0 as I][dir as I];                    // final dir of near atom
                if d1 != DIR_NO { set_then_dir(a, d1); }                  // distance between atoms is 1. update bond
            }
        }

        true
    }
    ///
    /// Implements fix command. Creates bond between two atoms. Consumes energy.
    ///
    pub fn atom_fix(&mut self, atom: Atom, vm_data: &mut  VMData) -> bool {
        true
    }
    ///
    /// Implements spl command. Splits two atoms. Releases energy.
    ///
    pub fn atom_spl(&mut self, atom: Atom, vm_data: &mut  VMData) -> bool {
        true
    }
    ///
    /// Implements cond command. Depending on the condition VM will run one of two
    /// possible atoms.
    ///
    pub fn atom_if(&mut self, atom: Atom, vm_data: &mut VMData) -> bool {
        true
    }
    ///
    /// Implements job command. Creates one new VM instance (thread).
    ///
    pub fn atom_job(&mut self, atom: Atom, vm_data: &mut VMData) -> bool {
        true
    }
    ///
    /// Just a stub for empty atom in a world
    ///
    fn atom_empty(&mut self, _atom: Atom, _vm_data: &mut VMData) -> bool { true }
}