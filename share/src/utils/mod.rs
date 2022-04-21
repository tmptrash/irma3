//!
//! Global utilities module
//!
use crate::{cfg::Config, global::Offs};
pub mod vec;
pub mod stack;
///
/// Private identifier. Is used in id() func
///
static mut ID: usize = 0;
///
/// Alias of unsafe {}
///
#[macro_export] macro_rules! u {
    ($arg:expr) => {
        unsafe { $arg }
    }
}
///
/// Generates unique id globally for entire app
///
pub fn id() -> String {
    unsafe {
        ID += 1;
        ID.to_string()
    }
}
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

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    use crate::utils::{*};

    fn create_file(file: &str, content: &str) {
        assert_eq!(fs::write(file, content).is_ok(), true);
    }
    fn remove_file(file: &str) {
        if Path::new(file).exists() {
            assert_eq!(fs::remove_file(file).is_ok(), true);
        }
    }

    #[test]
    fn test_id() {
        let id0 = id();
        let id1 = id();
        let id2 = id();
        assert!(id0 != id1 && id0 != id2 && id1 != id2);
    }
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
    #[test]
    fn test_to_offs() {
        let cfg_file = "def.json";
        create_file(cfg_file, r#"{"WIDTH": 10, "HEIGHT": 10}"#);

        let cfg = Config::new(cfg_file);
        assert_eq!(to_offs(0, 0, &cfg), 0);
        assert_eq!(to_offs(1, 0, &cfg), 1);
        assert_eq!(to_offs(1, 1, &cfg), 11);
        assert_eq!(to_offs(0, 2, &cfg), 20);
        assert_eq!(to_offs(9, 0, &cfg), 9);
        assert_eq!(to_offs(9, 9, &cfg), 99);

        remove_file(cfg_file);
    }
    #[test]
    fn test_to_xy() {
        let cfg_file = "def1.json";
        create_file(cfg_file, r#"{"WIDTH": 10, "HEIGHT": 10}"#);

        let cfg = Config::new(cfg_file);
        assert_eq!(to_xy(0, &cfg), (0, 0));
        assert_eq!(to_xy(1, &cfg), (1, 0));
        assert_eq!(to_xy(11, &cfg), (1, 1));
        assert_eq!(to_xy(20, &cfg), (0, 2));
        assert_eq!(to_xy(9, &cfg), (9, 0));
        assert_eq!(to_xy(99, &cfg), (9, 9));

        remove_file(cfg_file);
    }
}