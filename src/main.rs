mod world;
mod vm;
mod cfg;

use world::World;
use vm::VM;
use cfg::Config;

pub struct App {
    cfg: Config,
    world: World,
    vms: Vec<VM>
}

fn main() {
    let cfg = Config::new();
    let size = cfg.size();
    let app: App = App {
        cfg: cfg,
        world: World::new(size * 4),
        vms: vec![]
    };

    let index = 1024 * 1024 * 1024 * 4 - 1;
    println!("Welcome to irma4 - Artificial life simulator on Rust");
    println!("Allocated memory: {}. Value[{}]: {}", app.world.len(), index, app.world.get_dot(index));
}
