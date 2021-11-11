//!
//! A part of Virtual Machine module. Implements buffer, which is used for
//! moving molecules. Buffer should be reused (singleton) in all VMs.
//! 
use crate::utils;
use crate::global::Atom;
///
/// Buffer and stack of moving atoms, which are used by mov command.
/// Depending on it's size we may move big or small molecules. See mov
/// command implementation for details.
///
pub struct MoveBuffer {
    pub buf: Vec<Atom>,
    pub stack: Vec<Atom>
}

impl MoveBuffer {
    ///
    /// Creates new instance of a buffer. All VMs should use same reference to
    /// this buffer. It should be created only once.
    ///
    pub fn new(atoms: usize) -> MoveBuffer {
        MoveBuffer {
            buf: utils::alloc(atoms),
            stack: utils::alloc(atoms)
        }
    }
}