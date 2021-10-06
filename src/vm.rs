use std::mem;
use crate::world::World;
use crate::world::Atom;
use crate::utils;

pub struct VM {
    //
    // Energy in current VM. Every VM may have it's own energy
    //
    energy: usize,
    buf: Box<MoveBuffer>
}
//
// Buffer and stack of moving atoms, which are used for mov command. 
// Depending on it's size we may move big or small molecules
//
pub struct MoveBuffer {
    mov_buf: Vec<Atom>,
    mov_stack: Vec<Atom>
}

impl VM {
    pub fn new(buf: Box<MoveBuffer>) -> VM {
        VM {
            energy: 0,
            buf: buf
        }
    }

    pub fn mov(&mut self, mut world: &World) {

    }

    pub fn fix(&self, mut world: &World) {

    }

    pub fn cond(&self, mut world: &World) {

    }

    pub fn job(&self, mut world: &World) {

    }
}

impl MoveBuffer {
    pub fn new(atoms: usize) -> MoveBuffer {
        MoveBuffer {
            mov_buf: utils::alloc(atoms),
            mov_stack: utils::alloc(atoms)
        }
    }
}