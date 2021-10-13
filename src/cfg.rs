//
// Global configuration module. Is used to change config
// on a fly. Some values are read only (they are in upper 
// case), some read write (they are in lover case).
//
use getset::{CopyGetters, Getters};
//
// These annotations will create getters for readonly
// values like WIDTH, HEIGHT,...
//
#[derive(Getters, CopyGetters)]
#[getset(get_copy = "pub")]
#[allow(non_snake_case)]
pub struct Config {
    //
    // Readonly properties. Available through getters
    //
    WIDTH: usize,
    HEIGHT: usize,
    //
    // Size of moving buffers. We use these buffers in mov 
    // command to move atoms. Their size mean amount of atoms 
    // we may to move in one mov call
    //
    MOV_BUF_SIZE: usize,
    //
    // Maximum amount of VMs in a system
    //
    VM_AMOUNT: usize,
    //
    // Read-Write properties. Available through direct access
    // from every module
    //
    pub frame_delay: u32
}

impl Config {
    pub fn new() -> Config {
        Config {
            WIDTH: 1024,
            HEIGHT: 1024,
            MOV_BUF_SIZE: 1024,
            VM_AMOUNT: 1024,

            frame_delay : 0
        }
    }
}