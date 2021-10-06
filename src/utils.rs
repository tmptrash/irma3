//
// Allocates memory to specified size in bytes.
//
pub fn alloc<T>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    unsafe { v.set_len(len) }    
    v
}

pub fn zero<T>(vec: &mut Vec<T>, zero: T) where T: Copy {
    //
    // This peace of code init memory and allocate it, 
    // because alloc() doesn't really reserve the memory
    //
    for i in 0..vec.len() {
        vec[i] = zero;
    }
}