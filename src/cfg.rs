use getset::{CopyGetters, Getters};
//
// These annotations will create getters for readonly
// fileds like WIDTH, HEIGHT,...
//
#[derive(Getters, CopyGetters)]
#[getset(get_copy = "pub")]
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

            frame_delay : 0
        }
    }
}