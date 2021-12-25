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
mod plugins;

#[macro_use]
extern crate dlopen_derive;
use log::{*};
use colored::Colorize;
use world::World;
use vm::VM;
use vm::vmdata::VMData;
use vm::ret::Return;
use cfg::Config;
use vm::buf::MoveBuffer;
use share::global::DIR_REV;
use share::utils::vec::Vector;
use share::io::IO;
use share::io::events::{*};
///
/// Global configuration, which is shared for entire app. The meaning of this is
/// in ability to change RW properties in real time to affect application without
/// rerun
///
static mut CFG: Config = Config::new();
///
/// Inits core API. This is a place where Core adds listeners to different events,
/// which are fired from outside of the core. For example, from a plugin.
///
fn init_api(io: &mut IO) {
    info!("  Init core API");
    io.on(EVENT_RUN, |_|  {
        debug!("Run command catched");
        unsafe { CFG.is_running = !CFG.is_running };
    });
    io.on(EVENT_QUIT, |_| {
        debug!("Quit command catched");
        unsafe { CFG.stopped = true }
    });
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
/// and other Core components.
///
fn main() {
    flexi_logger::Logger::try_with_env_or_str("info").unwrap().format(flexi_logger::colored_opt_format).start().unwrap(); // use %RUST_LOG% to set log level. e.g.: SET RUST_LOG=info
    println!("\n{}\n", "Welcome to irma4 v0.1 - Atomic Artificial Life Simulator in Rust".green());
    info!("Init core");
    let mut io = IO::new(EVENT_LAST);
    init_api(&mut io);
    let plugins = plugins::load(unsafe { CFG.PLUGINS_DIR() });
    plugins::init(&plugins, &io);
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
        if i == 0 { plugins::idle(&plugins, &io) }
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

    plugins::remove(&plugins, &io);
}