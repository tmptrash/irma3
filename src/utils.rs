pub fn alloc<T>(bytes: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(bytes);
    unsafe { v.set_len(bytes) }
    v
}