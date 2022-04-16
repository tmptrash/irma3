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
#[derive(Serialize, Deserialize)]
pub struct AtomDump {
    pub a: Atom,
    pub x: isize,
    pub y: isize
}
///
/// Describes one VM in a world. x,y will be converted into offset
///
#[derive(Serialize, Deserialize)]
pub struct VmDump {
    pub x: isize,
    pub y: isize,
    pub e: isize
}
///
/// Describes one block of atoms and VMs. For example an organism
/// or molecule
///
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub atoms: Vec<AtomDump>,
    pub vms: Vec<VmDump>
}
///
/// Describes entiry dump of all atoms and VMs in a world
///
#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::{dump::Dump, cfg::Config, Core, utils::vec::Vector, io::IO};
    use crate::{vm::vmdata::VMData, global::ATOM_EMPTY};

    fn create_file(file: &str, content: &str) {
        assert_eq!(fs::write(file, content).is_ok(), true);
    }
    fn remove_file(file: &str) {
        if Path::new(file).exists() {
            assert_eq!(fs::remove_file(file).is_ok(), true);
        }
    }

    #[test]
    fn test_new() {
        let d = Dump::new();
        assert_eq!(d.width == 0 && d.height == 0, true);
        assert_eq!(d.blocks.len(), 0);
    }
    #[test]
    fn test_load() {
        let cfg_file = "load.json";
        create_file(cfg_file, r#"{"WIDTH": 10, "HEIGHT": 10}"#);

        let cfg = Config::new(cfg_file);
        let vm_amount = cfg.MAX_VM_AMOUNT();
        let width = cfg.WIDTH();
        let height = cfg.HEIGHT();
        let dir2offs = cfg.DIR_TO_OFFS();
        let mov_buf_size = cfg.MOV_BUF_SIZE();
        let mut core = Core {
            cfg,
            vms: Vector::new(vm_amount),
            io: IO::new(),
            vm_data: VMData::new(width, height, dir2offs, mov_buf_size)
        };
        let dump_file = "load.dump";
        create_file(dump_file, r#"{
            "width": 10,
            "height": 10,
            "blocks": [{
                "atoms": [{
                    "a": 58434,
                    "x": 0,
                    "y": 0
                }],
                "vms": [{
                    "x": 0,
                    "y": 0,
                    "e": 123
                }]
            }]
        }"#);
        assert_eq!(core.vm_data.world.get_atom(0), ATOM_EMPTY);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 0);
        assert_eq!(Dump::load(dump_file, &mut core), true);
        assert_eq!(core.vm_data.world.get_atom(0), 58434);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 1);

        remove_file(dump_file);
        remove_file(cfg_file);
    }
   #[test]
   fn test_load_no_file() {
        let cfg = Config::new("no_file.json"); // this file doesn't exist
        let vm_amount = cfg.MAX_VM_AMOUNT();
        let width = cfg.WIDTH();
        let height = cfg.HEIGHT();
        let dir2offs = cfg.DIR_TO_OFFS();
        let mov_buf_size = cfg.MOV_BUF_SIZE();
        let mut core = Core {
            cfg,
            vms: Vector::new(vm_amount),
            io: IO::new(),
            vm_data: VMData::new(width, height, dir2offs, mov_buf_size)
        };
        let file = "file.dump";
        remove_file(file);
        assert_eq!(core.vm_data.world.get_atom(0), ATOM_EMPTY);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 0);
        assert_eq!(Dump::load(file, &mut core), false);
        assert_eq!(core.vm_data.world.get_atom(0), ATOM_EMPTY);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 0);
    }
    #[test]
    fn test_load_bad_file() {
        let cfg_file = "bad.json";
        create_file(cfg_file, r#"{"WIDTH": 10, "HEIGHT": 10}"#);

        let cfg = Config::new(cfg_file);
        let vm_amount = cfg.MAX_VM_AMOUNT();
        let width = cfg.WIDTH();
        let height = cfg.HEIGHT();
        let dir2offs = cfg.DIR_TO_OFFS();
        let mov_buf_size = cfg.MOV_BUF_SIZE();
        let mut core = Core {
            cfg,
            vms: Vector::new(vm_amount),
            io: IO::new(),
            vm_data: VMData::new(width, height, dir2offs, mov_buf_size)
        };
        let dump_file = "bad.dump";
        create_file(dump_file, r#"{
            "width": 10,
            "height": 10,
            "blocks: [{
                "atoms": [{
                    "a": 58434,
                    "x": 0,
                    "y": 0
                }],
                "vms": [{
                    "x": 0,
                    "y": 0,
                    "e": 123
                }]
            }]
        }"#);
        assert_eq!(core.vm_data.world.get_atom(0), ATOM_EMPTY);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 0);
        assert_eq!(Dump::load(dump_file, &mut core), false);
        assert_eq!(core.vm_data.world.get_atom(0), ATOM_EMPTY);
        assert_eq!(core.vm_data.world.get_atom(1), ATOM_EMPTY);
        assert_eq!(core.vms.size(), 0);

        remove_file(dump_file);
        remove_file(cfg_file);
    }
}