mod world;
mod vm;
mod cfg;
mod utils;
mod global;

use std::io::stdin;
use world::World;
use vm::VM;
use vm::buf::MoveBuffer;
use cfg::Config;

pub struct App {
    world: World,
    vms: Vec<VM>
}

fn main() {
    //
    // Global configuration. Must be singleton
    //
    let mut cfg = Config::new();
    //
    // One move buffer for all VMs. Must be singleton
    //
    let buf = MoveBuffer::new(cfg.MOV_BUF_SIZE());
    let app: App = App {
        world: World::new(cfg.WIDTH() * cfg.HEIGHT()),
        vms: vec![VM::new(Box::new(buf))]
    };
    //
    // TODO: 
    //
    let index = cfg.WIDTH() * cfg.HEIGHT() - 1;
    println!("Welcome to irma4 - Artificial life simulator on Rust");
    println!("Allocated memory: {}. Value[{}]: {}", app.world.len(), index, app.world.get_dot(index));
    cfg.frame_delay = 123;
    println!("{}", cfg.frame_delay);
    //
    // keypress wait
    //
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
}
