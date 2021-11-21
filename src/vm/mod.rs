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
const ATOM_MAP: &'static [fn(&mut VM, Atom, &mut VMData) -> bool] = &[
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
    /// All possible directions of nearest atoms
    ///
    pub dirs: [i32; 8]
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
        if dir >= vm_data.dirs.len() as Dir {
            warn!("Invalid direction. Offs: {}, Atom: {}, Dir: {}", self.offs, atom, dir);
            return false;
        }
        self.offs += vm_data.dirs[dir as usize] as usize;

        ATOM_MAP[atom_type as usize](self, atom, vm_data)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms as well.
    ///
    pub fn atom_mov(&mut self, atom: Atom, vm_data: &mut  VMData) -> bool {
        vm_data.buf.stack.clear();
        vm_data.buf.buf.clear();
        vm_data.buf.stack.push(self.offs);
        self.atom_mov_inner(vm_data, (atom & ATOM_MOV_DIR >> ATOM_MOV_DIR_SHIFT) as Dir)
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
    ///
    /// This function rely that stack is not empty. check this before every call
    ///
    fn atom_mov_inner(&mut self, vm_data: &mut VMData, dir: Dir) -> bool {
        let mut offs: Offs;
        let mut atom: Atom;
        let mut near_offs: Offs;
        let mut near_atom: Atom;
        let mut d_offs: Offs;
        let stack = &mut vm_data.buf.stack;
        let world = &mut vm_data.world;
        let buf   = &mut vm_data.buf.buf;
        let dirs  = &vm_data.dirs;

        while stack.not_empty() {
            offs = stack.last();
            atom = world.get_dot(offs);
            near_offs = offs + dirs[dir as usize] as usize; // destination atom position
            near_atom = world.get_dot(near_offs);
            // Impossible to move near. Another atom is there, we have to move it first
            if is_atom(near_atom) { stack.push(near_offs); continue }

            stack.shrink();
            world.set_dot(near_offs, near_atom);
            buf.insert(near_offs);

            for d in 0..8 {
                d_offs = offs + dirs[d] as Offs;
                if buf.contains(&d_offs) { continue } // this atom has already moved
                World::is_near(offs, d_offs, world.width);
            }
        }

        true
    }
}