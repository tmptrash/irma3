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
mod plugins;

#[macro_use] extern crate dlopen_derive;
#[macro_use] extern crate share;
use log::{*};
use colored::Colorize;
use world::World;
use vm::VM;
use vm::vmdata::VMData;
use vm::ret::Return;
use vm::buf::MoveBuffer;
use share::cfg::Config;
use share::global::DIR_REV;
use share::utils::vec::Vector;
use share::io::IO;
use share::io::events::{*};
use share::logger;
use plugins::Plugins;
///
/// Global configuration, which is shared for entire app. The meaning of this is
/// in ability to change RW properties in real time to affect application without
/// rerun
///
static mut CFG: Config = Config::new();
///
/// Returns constant configuration value by property
///
macro_rules! cfgc { ($prop:ident) => { unsafe { CFG.$prop() } } }
///
/// Returns writable configuration value by property
///
macro_rules! cfgv { ($prop:ident) => { unsafe { CFG.$prop } } }
///
/// Shows a welcome string
///
fn show_welcome() {
    println!("\n{}\n", "Welcome to irma4 v0.1 - Atomic Artificial Life Simulator in Rust".green());
}
///
/// Shows bye message
///
fn show_bye() {
    println!("\n{}\n", "Bye".green());
}
///
/// Inits core API. This is a place where Core adds listeners to different events,
/// which are fired from outside of the core. For example, from a plugin.
///
fn init() -> IO {
    let mut io = IO::new(EVENT_LAST);
    
    logger::init();
    sec!("Init core API");
    io.on(EVENT_RUN, |_|  {
        dbg!("Run command catched");
        u!{ CFG.is_running = !CFG.is_running };
    });
    io.on(EVENT_QUIT, |_| {
        dbg!("Quit command catched");
        u! { CFG.stopped = true }
    });

    io
}
///
/// Creates a list of VMs.
///
fn create_vms(amount: usize) -> Vector<VM> {
    sec!("Create VMs");
    let mut vec = Vector::new(amount);
    for _i in 0..amount { vec.add(VM::new(0, 0)); }
    inf!("Created {} VMs", amount);
    vec
}
///
/// Creates VMData struct, which is used during VM work
///
fn create_vmdata(io: &IO) -> VMData {
    VMData {
        world: u! {World::new(CFG.WIDTH(), CFG.HEIGHT(), CFG.DIR_TO_OFFS()).unwrap()},
        buf: MoveBuffer::new(u! {CFG.MOV_BUF_SIZE()}),
        dirs_rev: DIR_REV,
        atoms_cfg: u! { &CFG.atoms },
        io
    }
}
///
/// Entry point of application. It creates global Configuration, World and list of VMs, logger
/// and other Core components.
///
fn main() {
    show_welcome();

    let io = init();
    let mut plugins = Plugins::new(&io);
    let mut vms = create_vms(cfgc!(VM_AMOUNT));
    let mut vm_data = create_vmdata(&io);

    plugins.load(cfgc!(PLUGINS_DIR));
    plugins.init(u! { &mut CFG });
    //
    // Main loop
    //
    inf!("Run main loop");
    if cfgc!(AUTORUN) { u!{CFG.is_running = cfgc!(AUTORUN) } }
    inf!("{}", if cfgc!(AUTORUN) { "Run" } else { "Waiting for a command..." });
    let mut i = 0;
    loop {
        if i == 0 { plugins.idle() }
        if cfgv!(stopped) { break }
        if !cfgc!(is_running) { continue }
        if vms.size() > 0 {
            if let Return::AddVm(energy, offs) = vms.data[i].run_atom(&mut vm_data) {
                if !vms.full() && vms.add(VM::new(energy, offs)) { vms.data[i].dec_energy(energy) }
            }
            if vms.data[i].get_energy() < 1 { vms.del(i); }

            i += 1;
            if i > vms.size() { i = 0 }
        }
    }

    plugins.remove();
    show_bye();
}