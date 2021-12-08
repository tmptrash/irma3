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