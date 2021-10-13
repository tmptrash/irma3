mod consts;
pub mod buf;

use std::rc::Rc;
use std::cell::RefCell;
use log::{*};
use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;
use consts::{*};
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOM_MAP: &'static [fn(&mut VM, &World) -> bool] = &[VM::atom_mov, VM::atom_fix, VM::atom_cond, VM::atom_job];

pub struct VM {
    //
    // Energy of current VM. Every VM may have it's own
    //
    energy: usize,
    //
    // Offset of current atom, which VM in running
    //
    offs: usize,
    //
    // Shared between VMs buffer. Is used in mov atom
    //
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

    pub fn create_vms(vm_amount: usize, buf_size: usize) -> Vec<VM> {
        let buf = Rc::new(RefCell::new(MoveBuffer::new(buf_size)));
        let mut v: Vec<VM> = Vec::with_capacity(vm_amount); 
        for _i in 0..vm_amount {
            v.push(VM::new(buf.clone()));
        }
        v
    }
    //
    // Run one atom
    //
    pub fn step(&mut self, mut world: &World) -> bool {
        let atom: Atom = world.get_dot(self.offs);
        let atom_type: usize = (atom & ATOM_TYPE_MASK).into();
        if atom_type >= ATOM_MAP.len() {
            warn!("Unsupported atom type. Offs: {}, Atom: {}, Type: {}", self.offs, atom, atom_type);
            return false;
        }
        ATOM_MAP[atom_type](self, world)
    }
    //
    // Atom handlers
    //
    pub fn atom_mov(&mut self, mut world: &World) -> bool {
        true
    }

    pub fn atom_fix(&mut self, mut world: &World) -> bool {
        true
    }

    pub fn atom_cond(&mut self, mut world: &World) -> bool {
        true
    }

    pub fn atom_job(&mut self, mut world: &World) -> bool {
        true
    }
}