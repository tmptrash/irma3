//!
//! A part of Virtual Machine module. Implements buffer, which is used for
//! moving molecules. Buffer should be reused (singleton) in all VMs.
//! 
use std::collections::HashSet;
use crate::utils;
use crate::global::Offs;
///
/// Means that stack has no elements in it.
///
pub const STACK_EMPTY: usize = usize::MAX;
///
/// Speed optimized stack. You have to check it's bounderies by hands to
/// reach max speed. It works with disabled overflow mode. It means that
/// if we reach zero stack size it's index (self.idx) will be STACK_EMPTY
///
pub struct Stack<T> {
    data: Vec<T>,
    idx: usize
}
///
/// Buffer and stack of moving atoms, which are used by mov command.
/// Depending on it's size we may move big or small molecules. See mov
/// command implementation for details.
///
pub struct MoveBuffer {
    pub buf: HashSet<Offs>,
    pub stack: Stack<Offs>
}

impl Stack<usize> {
    pub fn new(size: usize) -> Stack<Offs> {
        Stack {
            data: utils::alloc(size),
            idx: STACK_EMPTY
        }
    }
    ///
    /// Puts an atom offset to the stack's tail and increase stack size by 1. We dont
    /// need to check overflow for + operator, because we never reach usize::MAX - 1
    ///
    pub fn push(&mut self, offs: Offs) { self.idx += 1; self.data[self.idx] = offs }
    ///
    /// Returns atom's offs from the stack tail and reduce it's size by 1. We need overflow
    /// to get STACK_EMPTY in case of zero stack length
    ///
    pub fn pop(&mut self) -> Offs {
        let offs = self.data[self.idx];
        self.idx = self.idx.wrapping_sub(1);
        offs
    }
    ///
    /// Decrease stack size by 1. Doesn't return an atom' offs
    ///
    pub fn shrink(&mut self) { self.idx = self.idx.wrapping_sub(1) }
    ///
    /// Returns last atom's offs from stack without changing it's size
    ///
    pub fn last(&self) -> Offs { self.data[self.idx] }
    ///
    /// Fast reset of stack by moving index to the beginning
    ///
    pub fn reset(&mut self) { self.idx = STACK_EMPTY }
    ///
    /// Returns true, if stack contains at least one atom
    ///
    pub fn not_empty(&self) -> bool { self.idx != STACK_EMPTY }
}

impl MoveBuffer {
    ///
    /// Creates new instance of a buffer. All VMs should use same reference to
    /// this buffer. It should be created only once.
    ///
    pub fn new(size: usize) -> MoveBuffer {
        MoveBuffer {
            buf: HashSet::new(),
            stack: Stack::new(size)
        }
    }
}