//!
//! Implementation of fast vector. Has fixed amount of inner elements. Ability to
//! fast add, remove, iterate. Is used for a vector of Virtual Machines. Removed
//! elements will not be removed physically, just marked as removed. To reach fast
//! iteration through vector we keep active elements at the beginning of it. All
//! removed (marked) elements are in the tail. idx property pointing to the border
//! between active and removed elements. To iterate through vector use this code:
//!     let mut i: usize = 0;
//!     let v: Vector::new(10);
//!     v.add(1);
//!     v.add(2);
//!     while i < v.idx {
//!         ...
//!         i += 1;
//!     } 
//!
use crate::utils;

pub struct Vector<T: Copy> {
    ///
    /// Vector of elements
    ///
    pub data: Vec<T>,
    ///
    /// Index of last active element
    ///
    idx: usize,
    ///
    /// Amount of all (active + removed) elements in a vector
    ///
    size: usize
}

impl<T: Copy> Vector<T> {
    ///
    /// Returns new instance of a vector structure with predefined size
    ///
    pub fn new(size: usize) -> Vector<T> {
        Vector {
            data: utils::alloc(size),
            idx: 0,
            size
        }
    }

    pub fn add(&mut self, data: T) -> bool {
        if self.idx == self.size { return false }    // vector is full
        self.data[self.idx] = data;
        self.idx += 1;
        true
    }

    pub fn del(&mut self, index: usize) -> bool {
        if self.idx < 1 || index >= self.idx { return false }
        self.idx -= 1;
        self.data[index] = self.data[self.idx];
        true
    }

    pub fn size(&self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::vec::Vector;

    #[test]
    fn test_new() {
        let size = 2;
        let v: Vector<i32> = Vector::new(size);
        assert_eq!(v.size(), 0);
        assert_eq!(v.data.len(), size);
    }
    #[test]
    fn test_add() {
        let mut v: Vector<i32> = Vector::new(2);
        assert_eq!(v.add(1), true);
        assert_eq!(v.add(2), true);
        assert_eq!(v.add(3), false);
        assert_eq!(v.data[0] == 1 && v.data[1] == 2, true);
    }
    #[test]
    fn test_del() {
        let size = 3;
        let mut v: Vector<i32> = Vector::new(size);
        assert_eq!(v.size(), 0);
        assert_eq!(v.data.len(), size);
        v.add(1);
        v.add(2);
        v.add(3);
        assert_eq!(v.size(), size);
        assert_eq!(v.del(0), true);
        assert_eq!(v.data[0], 3);
        assert_eq!(v.size(), 2);
    }
    #[test]
    fn test_size() {
        let mut v: Vector<i32> = Vector::new(1);
        assert_eq!(v.size(), 0);
        v.add(1);
        assert_eq!(v.size(), 1);
        v.add(2);
        assert_eq!(v.size(), 1);
    }
    #[test]
    fn test_mixed() {
        let size = 2;
        let mut v: Vector<i32> = Vector::new(size);
        assert_eq!(v.size(), 0);
        v.add(1);                         // [1]
        v.add(2);                         // [1,2]
        v.add(3);                         // [1,2]
        assert_eq!(v.size(), size);
        v.del(1);                         // [1]
        assert_eq!(v.data[0], 1);
        v.del(1);                         // [1]
        assert_eq!(v.data[0], 1);
        v.del(0);                         // []
        assert_eq!(v.size(), 0);
        v.add(3);                         // [3]
        assert_eq!(v.size(), 1);
        assert_eq!(v.data[0], 3);         // [3]
    }
}