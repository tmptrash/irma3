//!
//! Module for loading and saving all atoms in a world
//!
///
/// Describes one atom in a world
///
use log::{*};
use crate::{err, vm::VM, global::Offs, utils::to_offs};
use serde::{Serialize, Deserialize};
use std::fs;
use crate::{global::Atom, Core};
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
    pub fn load(file: &str, core: &mut Core) -> bool {
        match fs::read_to_string(file) {
            Ok(s) => {
                match serde_json::from_str(&s) {
                    Ok(dump) => Dump::load_dump(file, dump, core),
                    Err(e) => {
                        err!("Error loading file \"{}\". Error: {}", file, e);
                        false
                    }
                }
            },
            Err(_) => {
                err!("Error loading file \"{}\"", file);
                false
            }
        }
    }
    ///
    /// Loads atoms and VMs from a dump file
    ///
    fn load_dump(file: &str, dump: Dump, core: &mut Core) -> bool {
        if dump.width != core.cfg.WIDTH() || dump.height != core.cfg.HEIGHT() {
            err!("Dump file \"{}\" has incorrect width and height. World size: {}x{}, Dump file size: {}x{}.",
                file,
                core.cfg.WIDTH(),
                core.cfg.HEIGHT(),
                dump.width,
                dump.height
            );
            return false;
        }
        let mut offs: Offs;
        let max_offs = core.cfg.WIDTH() * core.cfg.HEIGHT() - 1;
        for b in &dump.blocks {
            for a in &b.atoms {
                offs = to_offs(a.x, a.y, &core.cfg);
                if offs as usize > max_offs {
                    err!(
                        "Invalid atom position in a dump file. Atom: {}, Atom x,y: ({},{}), World size: ({},{})",
                        a.a, a.x, a.y, core.cfg.WIDTH(), core.cfg.HEIGHT()
                    );
                    continue;
                }
                core.vm_data.world.set_atom(offs, a.a, &core.io);
            }
            for vm in &b.vms {
                offs = to_offs(vm.x, vm.y, &core.cfg);
                if offs as usize > max_offs {
                    err!(
                        "Invalid VM position in a dump file. VM energy: {}, VM x,y: ({},{}), World size: ({},{})",
                        vm.e, vm.x, vm.y, core.cfg.WIDTH(), core.cfg.HEIGHT()
                    );
                    continue;
                }
                core.vms.add(VM::new(vm.e, to_offs(vm.x, vm.y, &core.cfg)));
            }
        }

        true
    }
}