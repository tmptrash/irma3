//!
//! Welcome to `irma4` - Atomic Artificial Life Simulator in Rust. This is stand alone
//! executable, which runs atomic physics in 2D world.
//!
mod world;
mod vm;
mod cfg;
mod utils;
mod global;
mod io;

use flexi_logger;
use log::{*};
use world::World;
use vm::VM;
use vm::vmdata::VMData;
use vm::ret::Return;
use cfg::Config;
use vm::buf::MoveBuffer;
use global::DIR_REV;
use utils::vec::Vector;
use io::IO;
///
/// Creates a list of VMs.
///
fn create_vms(amount: usize) -> Vector<VM> {
    let mut vec = Vector::new(amount);
    for _i in 0..amount { vec.add(VM::new(0, 0)); }
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
    let mut io  = IO::new();
    let mut vms = create_vms(cfg.VM_AMOUNT());
    let mut vm_data = VMData{                                                    // Only one instance of this struct must exist
        world: World::new(cfg.WIDTH(), cfg.HEIGHT(), cfg.DIR_TO_OFFS()).unwrap(),
        buf: MoveBuffer::new(cfg.MOV_BUF_SIZE()),
        dirs_rev: DIR_REV,
        atoms_cfg: &cfg.atoms,
        io: &io
    };
    //
    // Main loop
    //
    let mut i = 0;
    loop {
        if vms.size() > 0 {
            if let Return::AddVm(energy, offs) = vms.data[i].run_atom(&mut vm_data) {
                if vms.add(VM::new(energy, offs)) { vms.data[i].dec_energy(energy) }
            }
            if vms.data[i].get_energy() < 1 { vms.del(i); }

            i += 1;
            if i > vms.size() { i = 0 }
        }
    }
}