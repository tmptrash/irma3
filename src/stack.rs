//!
//! Speed optimized stack. You have to check it's bounderies by hands to
//! reach max speed. It works with disabled overflow mode. It means that
//! if we reach zero stack size it's index (self.idx) will be STACK_EMPTY.
//!
use crate::utils;
///
/// Means that stack has no elements in it.
///
pub const STACK_EMPTY: usize = usize::MAX;
///
/// Speed optimized stack.
///
pub struct Stack<T> {
    data: Vec<T>,
    idx: usize
}

impl<T: Copy> Stack<T> {
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            data: utils::alloc(size),
            idx: STACK_EMPTY
        }
    }
    ///
    /// Puts an atom offset to the stack's tail and increase stack size by 1. We dont
    /// need to check overflow for + operator, because we never reach usize::MAX - 1
    ///
    pub fn push(&mut self, offs: T) { self.idx += 1; self.data[self.idx] = offs }
    ///
    /// Returns atom's offs from the stack tail and reduce it's size by 1. We need overflow
    /// to get STACK_EMPTY in case of zero stack length
    ///
    pub fn pop(&mut self) -> T {
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
    pub fn last(&self) -> T { self.data[self.idx] }
    ///
    /// Fast reset of stack by moving index to the beginning
    ///
    pub fn clear(&mut self) { self.idx = STACK_EMPTY }
    ///
    /// Returns true, if stack contains at least one atom
    ///
    pub fn not_empty(&self) -> bool { self.idx != STACK_EMPTY }
}