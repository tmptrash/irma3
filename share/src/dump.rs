//!
//! Module for loading and saving all atoms in a world
//!
///
/// Describes one atom in a world
///
use log::{*};
use crate::{err, vm::VM, global::{Offs, ATOM_EMPTY}, utils::{to_offs, to_xy}};
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
    pub fn new() -> Dump {
        Dump {
            width: 0,
            height: 0,
            blocks: Vec::new()
        }
    }
    ///
    /// Saves a Dump struct into a file
    ///
    pub fn save(file: &str, core: &Core) -> bool {
        let mut dump = Dump::new();
        let mut offs: Offs = 0;
        let max_offs = (core.cfg.WIDTH() * core.cfg.HEIGHT()) as Offs;

        dump.width = core.cfg.WIDTH();
        dump.height = core.cfg.HEIGHT();
        let mut block = Block {atoms: Vec::new(), vms: Vec::new()};
        
        while offs < max_offs {
            let a = core.vm_data.world.get_atom(offs);
            if a != ATOM_EMPTY {
                let (x, y) = to_xy(offs, &core.cfg);
                block.atoms.push(AtomDump {a, x, y});
            }
            offs += 1;
        }
        for vm in &core.vms.data {
            let (x,y) = to_xy(vm.get_offs(), &core.cfg);
            block.vms.push(VmDump {x, y, e: vm.get_energy()});
        }
        dump.blocks.insert(0, block);

        match serde_json::to_string(&dump) {
            Ok(json) => {
                match fs::write(file, json) {
                    Ok(_) => true,
                    Err(_) => {
                        err!("Error saving file \"{}\"", file);
                        false
                    }
                }
            },
            Err(_) => {
                err!("Error saving file {}", file);
                false
            }
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