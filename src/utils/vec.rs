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
        let v: Vector<i32> = Vector::new(2);
        assert_eq!(v.size(), 0);
    }
}