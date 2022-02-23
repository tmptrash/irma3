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
mod plugins;
#[macro_use] mod defs;

#[macro_use] extern crate dlopen_derive;
#[macro_use] extern crate share;

use log::{*};
use colored::Colorize;
use share::cfg::Config;
use share::vm::vmdata::VMData;
use share::dump::Dump;
use share::utils::vec::Vector;
use share::io::{IO, Param, events::{*}};
use share::logger;
use share::Core;
use plugins::Plugins;
use defs::CORE;
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
/// Creates core and init it's API. This is a place where Core adds listeners
/// to different events, which fired from outside (from plugins).
///
fn init() {
    //
    // logger should be initialized before all inf!, err!, wrn!, dbg! macro calls
    //
    logger::init();
    sec!("Init core");
    let cfg = Config::new();
    let vm_amount = cfg.MAX_VM_AMOUNT();
    let width = cfg.WIDTH();
    let height = cfg.HEIGHT();
    let dir2offs = cfg.DIR_TO_OFFS();
    let mov_buf_size = cfg.MOV_BUF_SIZE();
    //
    // This is very important peace of code. Here we assign Core struct instance
    // to global CORE variable. It should be done only once in  a code and here
    //
    u! {
        CORE = Box::into_raw(Box::new(Core {
            cfg,
            vms: Vector::new(vm_amount),
            io: IO::new(),
            vm_data: VMData::new(width, height, dir2offs, mov_buf_size)
        })).cast()
    }
    
    inf!("Init core API");
    io!().on(EVENT_RUN, |_| {
        dbg!("\"Run\" command catched");
        u! { cfg!().is_running = !cfg!().is_running };
    });
    io!().on(EVENT_QUIT, |_| {
        dbg!("\"Quit\" command catched");
        cfg!().stopped = true;
    });
    io!().on(EVENT_LOAD_DUMP, |p: &Param| {
        if let Param::LoadAtoms(file) = p {
            dbg!("\"Load atoms\" command catched. Dump file: \"{}\"", file);
            Dump::load(file, core!());
        } else {
            err!("\"Load atoms\" command catched, but it contains wrong arguments. Should be Param::LoadAtoms(file)");
        }
    });
    io!().on(EVENT_SAVE_DUMP, |p: &Param| {
        if let Param::SaveAtoms(file) = p {
            dbg!("\"Save atoms\" command catched. Dump file: \"{}\"", file);
            Dump::save(file, core!());
        } else {
            err!("\"Save atoms\" command catched, but it contains wrong arguments. Should be Param::SaveAtoms(file)");
        }
    });
}
///
/// Entry point of application. It creates global Configuration, World and list of VMs, logger
/// and other Core components.
///
fn main() {
    show_welcome();

    init();
    let core = core!();
    let cfg = cfg!();
    let vms = vms!();
    let mut plugins = Plugins::new();
    
    plugins.load(cfg.PLUGINS_DIR());
    plugins.init(core);
    //
    // Main loop
    //
    inf!("Run main loop");
    if cfg.AUTORUN() { cfg.is_running = cfg.AUTORUN() }
    inf!("{}", if cfg.AUTORUN() { "Run" } else { "Waiting for a command..." });
    let mut i = 0;
    loop {
        if i == 0 { plugins.idle(core) }
        if cfg.stopped { break }
        if cfg.is_running { continue }
        if vms.size() > 0 {
            vms.data[i].run_atom(core);
            if vms.data[i].get_energy() < 1 { vms.del(i); }

            i += 1;
            if i >= vms.size() { i = 0 }
        }
    }

    plugins.remove(core);
    show_bye();
}