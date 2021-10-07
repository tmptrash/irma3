pub mod buf;

use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;

pub struct VM {
    //
    // Energy in current VM. Every VM may have it's own energy
    //
    energy: usize,
    buf: Box<MoveBuffer>,
    offs: usize
}

impl VM {
    pub fn new(buf: Box<MoveBuffer>) -> VM {
        VM {
            energy: 0,
            buf: buf,
            offs: 0
        }
    }
    pub fn create_vms(mov_buf_size: usize) -> Vec<VM> {
        let buf = MoveBuffer::new(mov_buf_size);
        // TODO:
        vec![VM::new(Box::new(buf))]
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