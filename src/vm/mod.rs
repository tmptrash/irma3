pub mod buf;

use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;

pub struct VM {
    //
    // Energy in current VM. Every VM may have it's own energy
    //
    energy: usize,
    offs: usize
}

impl VM {
    pub fn new() -> VM {
        VM {
            energy: 0,
            offs: 0
        }
    }
    pub fn create_vms(vm_amount: usize) -> Vec<VM> {
        // TODO: should be only one MoveBuffer
        vec![VM::new(), VM::new()]
    }
    //
    // Run one command/atom
    //
    pub fn step(&mut self, mut world: &World, mut buf: &MoveBuffer) {
        let atom = world.get_dot(self.offs);
    }

    pub fn cmd_mov(&mut self, mut world: &World, mut buf: &MoveBuffer) {

    }

    pub fn cmd_fix(&self, mut world: &World) {

    }

    pub fn cmd_cond(&self, mut world: &World) {

    }

    pub fn cmd_job(&self, mut world: &World) {

    }
}