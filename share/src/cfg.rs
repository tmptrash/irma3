//!
//! Global configuration module. Is used to change config on a fly. Some values
//! are read only (they are in upper case), some read write (they are in lover case).
//!
use getset::{CopyGetters, Getters};
use crate::{global::DIRS_LEN, inf};
use log::{*};
///
/// Configuration related to atoms
///
#[derive(Getters, CopyGetters)]
#[getset(get_copy = "pub")]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct AtomConfig {
    ///
    /// Amount of energy for moving one atom to one dot
    ///
    pub mov_energy: isize,
    ///
    /// Amount of energy for joining (fix) atoms
    ///
    pub fix_energy: isize,
    ///
    /// Amount of energy for splitting
    ///
    pub spl_energy: isize,
    ///
    /// Amount of energy for if atom
    ///
    pub if_energy: isize
}
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
    MAX_VM_AMOUNT: usize,
    ///
    /// Map of offsets depending on directions.
    /// 0 1 2
    /// 7 X 3
    /// 6 5 4
    /// 
    DIR_TO_OFFS: [i32; DIRS_LEN],
    ///
    /// Run system on start
    ///
    AUTORUN: bool,
    ///
    /// Folder of plugins (.dll or .so files)
    ///
    PLUGINS_DIR: &'static str,
    ///
    /// Current system run state
    ///
    pub is_running: bool,
    ///
    /// Indicate that app should to be stopped
    ///
    pub stopped: bool,
    ///
    /// Read-Write properties. Available through direct access from every module.
    ///
    pub frame_delay: u32,
    ///
    /// Amount of energy for moving one atom to one dot
    ///
    pub atoms: AtomConfig
}

impl Config {
    pub fn new() -> Config {
        inf!("Create global configuration");
        const WIDTH:  usize = 1920;
        const HEIGHT: usize = 1080;
        Config {
            // read only configuration
            WIDTH,
            HEIGHT,
            MOV_BUF_SIZE: 1024,
            MAX_VM_AMOUNT: 1024,
            DIR_TO_OFFS: [-(WIDTH as i32) - 1, -(WIDTH as i32), -(WIDTH as i32) + 1, 1, (WIDTH as i32) + 1, (WIDTH as i32), (WIDTH as i32) - 1, -1],
            AUTORUN: false,
            PLUGINS_DIR: "plugins",
            // read-write configuration
            is_running: false,
            stopped: false,
            frame_delay : 0,
            atoms: AtomConfig {
                mov_energy: 1,
                fix_energy: 1,
                spl_energy: 1,
                if_energy: 0
            }
        }
    }
}