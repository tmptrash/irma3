//!
//! Virtual Machine module. Implements all atom types and related stuff. Should be
//! optimized for speed. There are many virtual machines in a world at the same time.
//! One VM runs one molecule.
//! 
pub mod buf;

use std::convert::TryInto;
use log::{*};
use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;
use crate::global::{*};
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
    offs: usize
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
        let atom_type: usize = (atom & ATOM_TYPE_MASK >> ATOM_TYPE_SHIFT).try_into().unwrap();
        if atom_type == ATOM_EMPTY as usize { return false }

        let dir = (atom & ATOM_DIR_MASK >> ATOM_DIR_SHIFT) as usize;
        if dir >= vm_data.dirs.len() {
            warn!("Invalid direction. Offs: {}, Atom: {}, Dir: {}", self.offs, atom, dir);
            return false;
        }
        self.offs += vm_data.dirs[dir] as usize;

        ATOM_MAP[atom_type](self, atom, vm_data)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms as well.
    ///
    pub fn atom_mov(&mut self, atom: Atom, vm_data: &mut  VMData) -> bool {
        vm_data.buf.stack.push(atom);
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

    fn atom_mov_inner(&mut self, mut vm_data: &VMData) -> bool {
        for atom in vm_data.buf.stack.iter().rev() {
            // get
        }

        true
    }
}