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

fn main() {
    //
    // We use %RUST_LOG% to set log level. Use SET RUST_LOG=<level> in
    // terminal to set log level. e.g. SET RUST_LOG=info
    //
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    info!("Welcome to irma4 - Atomic Artificial Life Simulator in Rust");
    //
    // Global configuration. Must be a singleton
    //
    let mut cfg = Config::new();
    let mut world = World::new(cfg.WIDTH() * cfg.HEIGHT());
    let mut vms = VM::create_vms(cfg.VM_AMOUNT(), cfg.MOV_BUF_SIZE());
    //
    // TODO: 
    //
    vms[0].step(&world);
}