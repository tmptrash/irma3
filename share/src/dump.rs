//!
//! Module for loading and saving all atoms in a world
//!
///
/// Describes one atom in a world
///
use serde::{Serialize, Deserialize};
use std::fs;
use crate::global::Atom;
///
/// Describes an atom. x,y will be converted into offset
///
#[derive(Serialize, Deserialize, Debug)]
pub struct AtomDump {
    pub a: Atom,
    pub x: isize,
    pub y: isize
}
///
/// Describes one VM in a world. x,y will be converted into offset
///
#[derive(Serialize, Deserialize, Debug)]
pub struct VmDump {
    pub x: isize,
    pub y: isize,
    pub e: isize
}
///
/// Describes one block of atoms and VMs. For example an organism
/// or molecule
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub atoms: Vec<AtomDump>,
    pub vms: Vec<VmDump>
}
///
/// Describes entiry dump of all atoms and VMs in a world
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Dump {
    pub width: usize,                          // World width
    pub height: usize,                         // World height
    pub blocks: Vec<Block>                     // All atoms+VMs blocks
}

impl Dump {
    ///
    /// Saves a Dump struct into a file
    ///
    pub fn save(file: &str, dump: &Dump) -> Result<bool, String> {
        match serde_json::to_string(dump) {
            Ok(json) => {
                match fs::write(file, json) {
                    Ok(_) => Ok(true),
                    Err(_) => Err(format!("Error saving file {}", file))
                }
            },
            Err(_) => Err(format!("Error saving file {}", file))
        }
    }
    ///
    /// Loads dump from file and into a Dump struct
    ///
    pub fn load(file: &str) -> Result<Dump, String> {
        match fs::read_to_string(file) {
            Ok(s) => {
                match serde_json::from_str(&s) {
                    Ok(dump) => Ok(dump),
                    Err(_) => Err(format!("Error loading file \"{}\"", file))
                }
            },
            Err(_) => Err(format!("Error loading file \"{}\"", file))
        }
    }
}