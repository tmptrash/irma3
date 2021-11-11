//!
//! Global configuration module. Is used to change config on a fly. Some values
//! are read only (they are in upper case), some read write (they are in lover case).
//!
use getset::{CopyGetters, Getters};
//
// These annotations will create getters for readonly
// values like WIDTH, HEIGHT,...
//
#[derive(Getters, CopyGetters)]
#[getset(get_copy = "pub")]
#[allow(non_snake_case)]
pub struct Config {
    ///
    /// World width
    ///
    WIDTH: usize,
    ///
    /// World height
    /// 
    HEIGHT: usize,
    ///
    /// Size of moving buffers. We use these buffers in mov command to move
    /// atoms. Their size mean amount of atoms we may to move in one mov call.
    ///
    MOV_BUF_SIZE: usize,
    ///
    /// Maximum amount of VMs in a system.
    ///
    VM_AMOUNT: usize,
    ///
    /// Map of offsets depending on directions.
    /// 
    DIR_TO_OFFS: [i32; 8],
    ///
    /// Read-Write properties. Available through direct access from every module.
    ///
    pub frame_delay: u32
}

impl Config {
    pub fn new() -> Config {
        const width: usize = 1024;
        const height: usize = 1024;
        Config {
            WIDTH: width,
            HEIGHT: height,
            MOV_BUF_SIZE: 1024,
            VM_AMOUNT: 1024,
            DIR_TO_OFFS: [-(width as i32), -(width as i32) + 1, 1, (width as i32) + 1, (width as i32), (width as i32) - 1, -1, -(width as i32) - 1],

            frame_delay : 0
        }
    }
}