pub mod buf;

use std::rc::Rc;
use std::cell::RefCell;
use crate::world::World;
use crate::global::Atom;
use crate::utils;
use buf::MoveBuffer;

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
        let mut v: Vec<VM> = Vec::new();//utils::alloc(vm_amount);
        for _i in 0..vm_amount {
            v.push(VM::new(buf.clone()));
        }
        v
    }
    //
    // Run one command/atom
    //
    pub fn step(&mut self, mut world: &World) {
        let atom = world.get_dot(self.offs);
    }

    pub fn cmd_mov(&mut self, mut world: &World) {

    }

    pub fn cmd_fix(&self, mut world: &World) {

    }

    pub fn cmd_cond(&self, mut world: &World) {

    }

    pub fn cmd_job(&self, mut world: &World) {

    }
}