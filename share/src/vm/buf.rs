//!
//! A part of Virtual Machine module. Implements buffer, which is used for
//! moving molecules. Buffer should be reused (singleton) in all VMs.
//!
use log::{*};
use std::collections::HashSet;
use crate::global::Offs;
use crate::utils::stack::Stack;
use crate::inf;
///
/// Buffer and stack of moving atoms, which are used by mov command.
/// Depending on it's size we may move big or small molecules. See mov
/// command implementation for details.
///
pub struct MoveBuffer {
    pub buf: HashSet<Offs>,
    pub stack: Stack<Offs>
}

impl MoveBuffer {
    ///
    /// Creates new instance of a buffer. All VMs should use same reference to
    /// this buffer. It should be created only once.
    ///
    pub fn new(size: usize) -> MoveBuffer {
        inf!("Create MoveBuffer of size {}", size);
        MoveBuffer {
            buf: HashSet::new(),
            stack: Stack::new(size)
        }
    }
}