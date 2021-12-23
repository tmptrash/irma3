//!
//! Speed optimized stack. You have to check it's bounderies by hands to
//! reach max speed. It works with disabled overflow mode. It means that
//! if we reach zero stack size it's index (self.idx) will be STACK_EMPTY.
//!
use crate::utils;
use crate::global::I;
///
/// Means that stack has no elements in it.
///
pub const STACK_EMPTY: isize = -1;
///
/// Speed optimized stack.
///
pub struct Stack<T> {
    data: Vec<T>,
    idx: isize,
    size: isize
}

impl<T: Copy> Stack<T> {
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            data: utils::alloc(size),
            idx: STACK_EMPTY,
            size: size as isize
        }
    }
    ///
    /// Puts an elements to the stack's tail and increase stack size by 1
    ///
    pub fn push(&mut self, data: T) -> bool {
        if self.idx + 1 == self.size { return false }
        self.idx += 1;
        self.data[self.idx as I] = data;
        true
    }
    ///
    /// Returns element from the stack tail and reduce it's size by 1. We need overflow
    /// to get STACK_EMPTY in case of zero stack length
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.idx < 0 { return Option::None }
        let data = self.data[self.idx as I];
        self.idx -= 1;
        Option::Some(data)
    }
    ///
    /// Decrease stack size by 1. Doesn't return an element
    ///
    pub fn shrink(&mut self) { if self.idx < 0 { return }; self.idx -= 1; }
    ///
    /// Returns last element from stack without changing it's size
    ///
    pub fn last(&self) -> Option<T> {
        if self.idx < 0 { return Option::None }
        Option::Some(self.data[self.idx as I])
    }
    ///
    /// Fast reset of stack by moving index to the beginning
    ///
    pub fn clear(&mut self) { self.idx = STACK_EMPTY }
    ///
    /// Returns true, if stack contains at least one atom
    ///
    pub fn empty(&self) -> bool { self.idx == STACK_EMPTY }
}

#[cfg(test)]
mod tests {
    use crate::utils::stack::Stack;

    #[test]
    fn test_new() {
        let v: Stack<i32> = Stack::new(2);
        assert_eq!(v.empty(), true);
        assert_eq!(v.last(), Option::None);
    }
    #[test]
    fn test_push() {
        let mut v: Stack<i32> = Stack::new(2);
        v.push(1);
        assert_eq!(v.empty(), false);
        assert_eq!(v.last().unwrap(), 1);
        v.push(2);
        assert_eq!(v.empty(), false);
        assert_eq!(v.last().unwrap(), 2);
        assert_eq!(v.push(3), false);
        assert_eq!(v.empty(), false);
        assert_eq!(v.last().unwrap(), 2);
    }
    #[test]
    fn test_pop() {
        let mut v: Stack<i32> = Stack::new(2);
        assert_eq!(v.empty(), true);
        v.push(1);
        assert_eq!(v.empty(), false);
        assert_eq!(v.pop().unwrap(), 1);
        assert_eq!(v.empty(), true);
        assert_eq!(v.pop(), Option::None);
        assert_eq!(v.empty(), true);
    }
    #[test]
    fn test_pop1() {
        let mut v: Stack<i32> = Stack::new(2);
        assert_eq!(v.pop(), Option::None);
        assert_eq!(v.push(1), true);
        assert_eq!(v.pop().unwrap(), 1);
        assert_eq!(v.pop(), Option::None);
        assert_eq!(v.empty(), true);
    }
    #[test]
    fn test_shrink() {
        let mut v: Stack<i32> = Stack::new(2);
        v.push(1);
        v.push(2);
        v.shrink();
        assert_eq!(v.last().unwrap(), 1);
        assert_eq!(v.empty(), false);
        v.shrink();
        assert_eq!(v.last(), Option::None);
        assert_eq!(v.empty(), true);
    }
    #[test]
    fn test_last() {
        let mut v: Stack<i32> = Stack::new(3);
        v.push(3);
        v.push(2);
        v.push(1);
        assert_eq!(v.last().unwrap(), 1);
        assert_eq!(v.last().unwrap(), 1);
        v.pop();
        assert_eq!(v.last().unwrap(), 2);
        v.pop();
        assert_eq!(v.last().unwrap(), 3);
        assert_eq!(v.empty(), false);
        v.pop();
        assert_eq!(v.empty(), true);
        assert_eq!(v.last(), Option::None);
    }
    #[test]
    fn test_clear() {
        let mut v: Stack<i32> = Stack::new(3);
        assert_eq!(v.empty(), true);
        v.clear();
        assert_eq!(v.empty(), true);
        v.push(1);
        assert_eq!(v.empty(), false);
        v.clear();
        assert_eq!(v.empty(), true);
        v.push(1);
        v.push(2);
        assert_eq!(v.empty(), false);
        assert_eq!(v.last().unwrap(), 2);
        v.clear();
        assert_eq!(v.empty(), true);
        assert_eq!(v.last(), Option::None);
    }
    #[test]
    fn test_empty() {
        let mut v: Stack<i32> = Stack::new(2);
        assert_eq!(v.empty(), true);
        v.push(1);
        assert_eq!(v.empty(), false);
        v.pop();
        assert_eq!(v.empty(), true);
        v.push(1);
        v.push(2);
        assert_eq!(v.empty(), false);
        v.shrink();
        assert_eq!(v.empty(), false);
        v.shrink();
        assert_eq!(v.empty(), true);
    }
    #[test]
    fn test_mixed() {
        let mut v1: Stack<i32> = Stack::new(2);
        let mut v2: Stack<i32> = Stack::new(2);
        assert_eq!(v1.empty(), true);
        assert_eq!(v2.empty(), true);
        v1.push(1);
        assert_eq!(v1.empty(), false);
        assert_eq!(v2.empty(), true);
        v2.push(2);
        assert_eq!(v1.empty(), false);
        assert_eq!(v2.empty(), false);
        v1.pop();
        assert_eq!(v1.empty(), true);
        assert_eq!(v2.empty(), false);
        v1.push(1);
        v2.push(2);
        v1.clear();
        v2.clear();
        assert_eq!(v1.empty(), true);
        assert_eq!(v2.empty(), true);
        v1.push(1);
        v2.push(2);
        v2.shrink();
        assert_eq!(v1.empty(), false);
        assert_eq!(v2.empty(), true);
        v2.shrink();
        assert_eq!(v1.empty(), false);
        assert_eq!(v2.empty(), true);
        assert_eq!(v1.last().unwrap(), 1);
    }
}