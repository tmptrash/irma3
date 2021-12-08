//!
//! Welcome to `irma4` - Atomic Artificial Life Simulator in Rust. This is stand alone
//! executable, which runs atomic physics of plain rectangular area in 2D world.
//!
mod world;
mod vm;
mod cfg;
mod utils;
mod global;
mod stack;
mod vec;

use flexi_logger;
use log::{*};
use world::World;
use vm::VM;
use vm::VMData;
use cfg::Config;
use vm::buf::MoveBuffer;
use global::DIR_REV;
use vec::Vector;

fn add_vm() -> bool { true }
///
/// Creates a list of VMs.
///
fn create_vms(amount: usize) -> Vector<VM> {
    let mut vec = Vector::new(amount);
    for _i in 0..amount { vec.add(VM::new()); }
    vec
}
///
/// Entry point of application. It creates global Configuration, World and list of VMs, logger
/// and other components.
///
fn main() {
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();              // use %RUST_LOG% to set log level. e.g.: SET RUST_LOG=info
    info!("Welcome to irma4 - Atomic Artificial Life Simulator in Rust");

    let mut cfg = Config::new();                                                 // Global configuration. Must be a singleton
    let mut vms = create_vms(cfg.VM_AMOUNT());
    let mut vm_data = VMData{                                                    // Only one instance of this struct must exist
        world: World::new(cfg.WIDTH(), cfg.HEIGHT(), cfg.DIR_TO_OFFS()).unwrap(),
        buf: MoveBuffer::new(cfg.MOV_BUF_SIZE()),
        dirs_rev: DIR_REV,
        atoms_cfg: cfg.atoms,
        add_vm: add_vm
    };
    //
    // TODO: should be a loop over VMs
    // TODO: should be a check if energy < 1 to remove VM
    //
    //if self.energy < 1 { return false }
    vms.data[0].run_atom(&mut vm_data);
}