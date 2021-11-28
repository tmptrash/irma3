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
use crate::utils;
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOMS_MAP: &'static [fn(&mut VM, Atom, &mut VMData) -> bool] = &[
    VM::atom_empty,  // must be an empty fn. Means empty cell or no atom
    VM::atom_mov,
    VM::atom_fix,
    VM::atom_spl,
    VM::atom_cond,
    VM::atom_job,
    VM::atom_empty,  // unused
    VM::atom_empty   // unused
];
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

        ATOMS_MAP[atom_type as usize](self, atom, vm_data)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms together.
    /// Should be optimized by speed. After moving all bonds should not be broken.
    ///
    pub fn atom_mov(&mut self, atom: Atom, vm_data: &mut VMData) -> bool {
        let mut offs: Offs;
        let mut atom: Atom;
        let mut to_offs: Offs;
        let mut to_atom: Atom;
        let mut d_offs: Offs;
        let dir = (atom & ATOM_MOV_DIR >> ATOM_MOV_DIR_SHIFT) as Dir;
        let stack = &mut vm_data.buf.stack;
        let world = &mut vm_data.world;
        let buf   = &mut vm_data.buf.buf;
        let world_width = world.width;

        stack.clear();                                            // every call of mov should reset stack & buf
        buf.clear();
        stack.push(self.offs);

        while stack.not_empty() {                                 // before while, stack should have >= 1 atom
            offs = stack.last();
            atom = world.get_dot(offs);                           // atom we have to move
            to_offs = world.get_offs(offs, dir);                  // destination atom position
            to_atom = world.get_dot(to_offs);
            if is_atom(to_atom) { stack.push(to_offs); continue } // can't move atom. Another one is there

            stack.shrink();
            world.mov_dot(offs, to_offs, atom);
            buf.insert(to_offs);

            for d in 0..DIRS_LEN as Dir {
                d_offs = world.get_offs(offs, d);
                if buf.contains(&d_offs) { continue }             // this atom has already moved
                
                if get_vm_dir(atom) == d {                        // update moved atom's bond
                }
                if World::is_near(to_offs, d_offs, world_width) {
                    // TODO: check dir of d_atom and atom
                    let d_atom = set_vm_dir(world.get_dot(d_offs), );
                    world.set_dot(d_offs, d_atom);
                } else {
                    stack.push(d_offs);
                }
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
    pub fn atom_cond(&mut self, atom: Atom, vm_data: &mut VMData) -> bool {
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