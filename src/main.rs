//!
//! `irma4` is a 2D simulator of virtual particles, which was created to produce `Artificial Life` 
//! forms. Due to the fact, that our universe is very complex and requires an enormous amount of 
//! computational resources, the creation of a more or less realistic virtual world (and later, 
//! virtual life) is an almost impossible task today. So we as scientists and developers are trying 
//! to find the most similar and also simple model, which is computable for modern PCs. irma4 is a
//!  mix of [Artificial Chemistry](https://en.m.wikipedia.org/wiki/Artificial_chemistry), 
//! [two](https://esolangs.org/wiki/Category:Two-dimensional_languages)-dimensional programming 
//! [language](https://en.m.wikipedia.org/wiki/Programming_language) and simple 
//! [particle physics](https://en.m.wikipedia.org/wiki/Particle_physics). It's an experiment with 
//! unpredictable results. Also, there is an idea of 
//! [Open-Ended Evolution](https://royalsocietypublishing.org/doi/10.1098/rsif.2018.0395#:~:text=10.1098%2Frsif.2018.0395-,Abstract,characterize%20evolution%20on%20multiple%20scales), 
//! which refers to the unbounded increase in complexity that seems to characterize evolution on 
//! multiple scales. The system starts from very simple elements and their interactions and increases 
//! its complexity almost infinitely. This is actually how life variety appears. We hope, that 
//! such a process will partially appear in our system as well. This is actually fourth 
//! ([first](https://github.com/tmptrash/jevo), [second](https://github.com/tmptrash/construct), 
//! [third](https://github.com/tmptrash/irma)) attempt to create such simulator, so we have some 
//! experience in the area. The general idea is very similar to real-world, but with many 
//! restrictions: there is a 2D world of [atoms](#Atoms) (colored pixels). They may join together 
//! to create [molecules](#Molecules) and simple organisms. There are few interactions and atoms 
//! types, which give all the variety of forms in this virtual world. To run such interactions 
//! (we also call them "run atoms") we use special [Virtual Machines](#Atomic-Virtual-Machines).
//!
mod world;
mod vm;
mod cfg;
mod utils;
mod global;
mod io;
mod plugins;

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
use io::Param;
use io::events::{*};
use plugins::terminal;
///
/// Global configuration, which is shared for entire app
///
static mut CFG: Config = Config::new();
///
/// Inits core API
///
fn init_api(io: &mut IO) {
    info!("  Init core API");
    io.on(EVENT_RUN, |p: &Param| { if let Param::Run(run) = p { unsafe { CFG.is_running = *run } } });
}
///
/// Init plugins of the core
///
fn init_plugins(io: &mut IO) {
    info!("  Init core plugins");
    terminal::init(io);
}
///
/// Destroy plugins of the core
///
fn destroy_plugins(io: &IO) {
    info!("  Destroy core plugins");
    terminal::destroy(io);
}
///
/// Call plugins idle() function to do their internal work
///
fn idle_pugins(io: &IO) {
    terminal::idle(io);
}
///
/// Creates a list of VMs.
///
fn create_vms(amount: usize) -> Vector<VM> {
    info!("  Create VMs");
    let mut vec = Vector::new(amount);
    for _i in 0..amount { vec.add(VM::new(0, 0)); }
    info!("    Created {} VMs", amount);
    vec
}
///
/// Entry point of application. It creates global Configuration, World and list of VMs, logger
/// and other core components.
///
fn main() {
    flexi_logger::Logger::try_with_env().unwrap().format(flexi_logger::colored_opt_format).start().unwrap();              // use %RUST_LOG% to set log level. e.g.: SET RUST_LOG=info
    info!("Welcome to irma4 v0.1 - Atomic Artificial Life Simulator in Rust");
    info!("Init core");
    let mut io  = IO::new();
    init_api(&mut io);
    init_plugins(&mut io);
    let mut vms = create_vms(unsafe { CFG.VM_AMOUNT() });
    info!("  Create world");
    let mut vm_data = VMData {                                                   // Only one instance of this struct must exist
        world: unsafe {World::new(CFG.WIDTH(), CFG.HEIGHT(), CFG.DIR_TO_OFFS()).unwrap()},
        buf: MoveBuffer::new(unsafe {CFG.MOV_BUF_SIZE()}),
        dirs_rev: DIR_REV,
        atoms_cfg: unsafe { &CFG.atoms },
        io: &io
    };
    //
    // Main loop
    //
    unsafe { if CFG.AUTORUN() { CFG.is_running = CFG.AUTORUN() } }
    info!("{}", if unsafe {CFG.AUTORUN()} { "Run" } else { "Waiting for a command..." });
    let mut i = 0;
    loop {
        if i == 0 { idle_pugins(&io) }
        if unsafe { CFG.stopped } { break }
        if unsafe { !CFG.is_running } { continue }
        if vms.size() > 0 {
            if let Return::AddVm(energy, offs) = vms.data[i].run_atom(&mut vm_data) {
                if !vms.full() && vms.add(VM::new(energy, offs)) { vms.data[i].dec_energy(energy) }
            }
            if vms.data[i].get_energy() < 1 { vms.del(i); }

            i += 1;
            if i > vms.size() { i = 0 }
        }
    }

    destroy_plugins(&io);
}