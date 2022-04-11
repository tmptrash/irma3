//!
//! Global utilities module
//!
use crate::{cfg::Config, global::Offs};
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
    for val in vec.iter_mut() { *val = zero }
}
///
/// Converts x,y into Offs
///
pub fn to_offs(x: isize, y: isize, cfg: &Config) -> Offs {
    y * cfg.WIDTH() as isize + x
}
///
/// Converts offset into x,y
///
pub fn to_xy(offs: Offs, cfg: &Config) -> (isize, isize) {
    (offs % cfg.WIDTH() as isize, offs / cfg.WIDTH() as isize)
}
///
/// Alias of unsafe {}
///
#[macro_export] macro_rules! u {
    ($arg:expr) => {
        unsafe { $arg }
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
    #[test]
    fn test_zero1() {
        let mut v: Vec<u32> = alloc(2);
        zero(&mut v, 1);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 1);
    }
    #[test]
    fn test_zero2() {
        let size = 1024 * 1024;
        let mut v: Vec<u32> = alloc(size);
        zero(&mut v, 1);
        assert_eq!(v.len(), size);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 1);
        assert_eq!(v[size - 1], 1);
    }
}