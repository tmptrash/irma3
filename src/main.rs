//!
//! Welcome to `irma4` - Atomic Artificial Life Simulator in Rust. This is stand alone
//! executable, which runs atomic physics of plain rectangular area in 2D world.
//!
mod world;
mod vm;
mod cfg;
mod utils;
mod global;

use flexi_logger;
use log::{*};
use world::World;
use vm::VM;
use cfg::Config;
///
/// Entry point of application. It creates global Configuration, World and list of VMs, logger
/// and other components.
///
fn main() {
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();              // use %RUST_LOG% to set log level. e.g.: SET RUST_LOG=info
    info!("Welcome to irma4 - Atomic Artificial Life Simulator in Rust");

    let mut cfg = Config::new();                                                 // Global configuration. Must be a singleton
    let mut world = World::new(cfg.WIDTH() * cfg.HEIGHT()).unwrap();
    let mut vms = VM::create_vms(cfg.VM_AMOUNT(), cfg.MOV_BUF_SIZE());
    //
    // TODO: 
    //
    vms[0].run_atom(&world, &cfg.DIR_TO_OFFS());
}