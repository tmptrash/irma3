use std::mem;
//
// Allocates memory to specified size in bytes.
//
pub fn alloc<T>(len: usize, zero: T) -> Vec<T> where T: Copy {
    let mut v = Vec::with_capacity(len);
    unsafe { v.set_len(len) }    
    //
    // This peace of code init memory and allocate it, 
    // because alloc() doesn't really reserve the memory
    //
    for i in 0..len {
        v[i] = zero;
    }
    v
}