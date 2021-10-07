use crate::utils;
use crate::global::Atom;
//
// Buffer and stack of moving atoms, which are used for mov command.
// Depending on it's size we may move big or small molecules
//
pub struct MoveBuffer {
    buf: Vec<Atom>,
    stack: Vec<Atom>
}

impl MoveBuffer {
    pub fn new(atoms: usize) -> MoveBuffer {
        MoveBuffer {
            buf: utils::alloc(atoms),
            stack: utils::alloc(atoms)
        }
    }
}