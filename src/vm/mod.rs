//!
//! Virtual Machine module. Implements all atom types and related stuff. Should be
//! optimized for speed. There are many virtual machines in a world at the same time.
//! One VM runs one molecule.
//! 
pub mod buf;

use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;
use log::{*};
use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;
use crate::global::{*};
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOM_MAP: &'static [fn(&mut VM, &World) -> bool] = &[
    VM::atom_empty,  // must be an empty fn. Means empty cell or no atom
    VM::atom_mov,
    VM::atom_fix,
    VM::atom_spl,
    VM::atom_cond,
    VM::atom_job,
    VM::atom_empty,  // unused
    VM::atom_empty   // unused
];

pub struct VM {
    ///
    /// Energy of current VM. Every VM may have it's own.
    ///
    energy: usize,
    ///
    /// Offset of current atom, which VM in running.
    ///
    offs: usize,
    ///
    /// Shared between VMs buffer. Is used in mov atom.
    ///
    buf: Rc<RefCell<MoveBuffer>>
}

impl VM {
    pub fn new(buf: Rc<RefCell<MoveBuffer>>) -> VM {
        VM {
            energy: 0,
            buf: buf,
            offs: 0
        }
    }
    ///
    /// Creates a list of VMs. It's size depends on configuration.
    ///
    pub fn create_vms(vm_amount: usize, buf_size: usize) -> Vec<VM> {
        let buf = Rc::new(RefCell::new(MoveBuffer::new(buf_size)));
        let mut v: Vec<VM> = Vec::with_capacity(vm_amount); 
        for _i in 0..vm_amount {
            v.push(VM::new(buf.clone()));
        }
        v
    }
    ///
    /// Runs one atom depending on type and moves VM to the next one depending on
    /// atom direction.
    ///
    pub fn run_atom(&mut self, mut world: &World, dir_to_offs: &[i32]) -> bool {
        let atom: Atom = world.get_dot(self.offs);
        let atom_type: usize = (atom & ATOM_TYPE_MASK >> ATOM_TYPE_SHIFT).try_into().unwrap();
        if atom_type == ATOM_EMPTY as usize { return false }

        let dir = (atom & ATOM_DIR_MASK >> ATOM_DIR_SHIFT) as usize;
        if dir >= dir_to_offs.len() {
            warn!("Invalid direction. Offs: {}, Atom: {}, Dir: {}", self.offs, atom, dir);
            return false;
        }
        self.offs += dir_to_offs[dir] as usize;

        ATOM_MAP[atom_type](self, world)
    }
    ///
    /// Implements mov command. It moves current atom and all binded atoms as well.
    ///
    pub fn atom_mov(&mut self, mut world: &World) -> bool {
        true
    }
    ///
    /// Implements fix command. Creates bond between two atoms. Consumes energy.
    ///
    pub fn atom_fix(&mut self, mut world: &World) -> bool {
        true
    }
    ///
    /// Implements spl command. Splits two atoms. Releases energy.
    ///
    pub fn atom_spl(&mut self, mut world: &World) -> bool {
        true
    }
    ///
    /// Implements cond command. Depending on the condition VM will run one of two
    /// possible atoms.
    ///
    pub fn atom_cond(&mut self, mut world: &World) -> bool {
        true
    }
    ///
    /// Implements job command. Creates one new VM instance (thread).
    ///
    pub fn atom_job(&mut self, mut world: &World) -> bool {
        true
    }
    ///
    /// Just a stub for empty atom in a world
    ///
    fn atom_empty(&mut self, mut _world: &World) -> bool { true }
}