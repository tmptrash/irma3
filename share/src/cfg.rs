//!
//! Global configuration module. Is used to change config on a fly. Some values
//! are read only (they are in upper case), some read write (they are in lover case).
//!
use getset::{CopyGetters, Getters};
use serde_json::{Value};
use crate::{global::DIRS_LEN, inf, err};
use log::{*};
use std::fs;
use serde::{Serialize, Deserialize};
///
/// Name of default configuration file, which is used to load
/// configuration and apply it into Config struct. It will be
/// used within irma
///
pub const CONFIG_FILE: &str = "config.json";

///
/// Configuration related to atoms
///
#[derive(Getters, CopyGetters)]
#[getset(get_copy = "pub")]
#[allow(non_snake_case)]
#[derive(Copy, Clone, Serialize, Deserialize)]
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
#[derive(Getters, CopyGetters, Serialize, Deserialize)]
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
    pub fn new(file: &str) -> Config {
        inf!("Read {}", file);
        let cfg: Value = match fs::read_to_string(file) {
            Ok(s) => {
                match serde_json::from_str(&s) {
                    Ok(val) => val,
                    Err(e) => {
                        err!("Error loading file \"{}\". Error: {}", file, e);
                        Value::Null
                    }
                }
            },
            Err(_) => {
                err!("Error loading file \"{}\"", file);
                Value::Null
            }
        };

        inf!("Create global configuration");
        let width:  usize = Config::to_i64(&cfg["WIDTH"], 10) as usize;
        let height: usize = Config::to_i64(&cfg["HEIGHT"], 10) as usize;

        Config {
            // read only configuration
            WIDTH: width,
            HEIGHT: height,
            MOV_BUF_SIZE: Config::to_i64(&cfg["MOV_BUF_SIZE"], 1024) as usize,
            MAX_VM_AMOUNT: Config::to_i64(&cfg["MAX_VM_AMOUNT"], 1024) as usize,
            DIR_TO_OFFS: Config::get_dir_offs(width as i32),
            AUTORUN: Config::to_bool(&cfg["AUTORUN"], false),
            PLUGINS_DIR: "plugins",
            // read-write configuration
            is_running: Config::to_bool(&cfg["is_running"], false),
            stopped: Config::to_bool(&cfg["stopped"], false),
            frame_delay: Config::to_i64(&cfg["frame_delay"], 0) as u32,
            atoms: AtomConfig {
                mov_energy: Config::to_i64(&cfg["mov_energy"], 1) as isize,
                fix_energy: Config::to_i64(&cfg["fix_energy"], 1) as isize,
                spl_energy: Config::to_i64(&cfg["spl_energy"], 1) as isize,
                if_energy: Config::to_i64(&cfg["if_energy"], 0) as isize,
            }
        }
    }
    ///
    /// Returns direction offsets array
    ///
    pub fn get_dir_offs(w: i32) -> [i32; 8] { [-w - 1, -w, -w + 1, 1, w + 1, w, w - 1, -1] }
    ///
    /// Returns i64 value from a map or default value
    ///
    fn to_i64(val: &Value, def: i64) -> i64 {
        let v = val.as_i64();
        if v.is_none() { return def }
        v.unwrap()
    }
    ///
    /// Returns String value from a map or default value
    ///
    // fn to_string(val: &Value, def: &str) -> String {
    //     let v = val.as_str();
    //     if v.is_none() { return def.to_string() }
    //     v.unwrap().to_string()
    // }
    ///
    /// Returns bool value from a map or default value
    ///
    fn to_bool(val: &Value, def: bool) -> bool {
        let v = val.as_bool();
        if v.is_none() { return def }
        v.unwrap()
    }
}