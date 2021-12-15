//!
//! Global utilities module
//!
pub mod vec;
pub mod stack;
///
/// Allocates memory to specified size in bytes.
///
pub fn alloc<T>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    unsafe { v.set_len(len) }
    v
}
///
/// Zeroes vector.
///
pub fn zero<T: Copy>(vec: &mut Vec<T>, zero: T) {
    for i in 0..vec.len() {
        vec[i] = zero;
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{*};

    #[test]
    fn test_alloc() {
        let mut v: Vec<u32> = alloc(2);
        v[0] = 1;
        v[1] = 2;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
    }
    #[test]
    fn test_zero() {
        let mut v: Vec<u32> = alloc(3);
        zero(&mut v, 0);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 0);
        assert_eq!(v[2], 0);
    }
}