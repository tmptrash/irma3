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

pub struct App {
    world: World,
    vms: Vec<VM>
}

fn main() {
    //
    // We use %RUST_LOG% to set log level. Use SET RUST_LOG=<level> in
    // terminal to set log level. e.g. SET RUST_LOG=info
    //
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    //
    // Global configuration. Must be singleton
    //
    let mut cfg = Config::new();
    let mut app: App = App {
        world: World::new(cfg.WIDTH() * cfg.HEIGHT()),
        vms: VM::create_vms(cfg.VM_AMOUNT(), cfg.MOV_BUF_SIZE())
    };
    //
    // TODO: 
    //
    let index = cfg.WIDTH() * cfg.HEIGHT() - 1;
    info!("Welcome to irma4 - Artificial life simulator on Rust");
    info!("Allocated memory: {}. Value[{}]: {}", app.world.len(), index, app.world.get_dot(index));
    cfg.frame_delay = 123;
    println!("{}", cfg.frame_delay);
    //
    // TODO:
    //
    app.vms[0].step(&app.world);
}