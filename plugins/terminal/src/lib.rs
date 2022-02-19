//!
//! Implementation of terminal plugin. Gives an ability to run terminal commands
//! during app execution.
//!
use std::{thread, io};
use std::sync::{mpsc, mpsc::Receiver, mpsc::TryRecvError};
use colored::Colorize;
use share::io::Param;
use share::io::events::{*};
use share::Core;
use share::u;
//
// Local variable of this module, which keeps thread instance
//
thread_local!(static THREAD: Receiver<String> = create_thread());
///
/// Local variable, which affects thread activeness
///
static mut THREAD_STOPPED: bool = false;
///
/// Suported command and arguments separators. For example: load file.txt |
/// load=file.txt | l file.txt | l=file.txt
///
const CMD_SEPARATOR: &[char; 3] = &[' ','=',':'];
///
/// Help message
///
const HELP_MSG: &str = "Supported commands:
    q, quit       Quit the system
    r, run        Run or stop the system
    h, help       Show this message
    l, load=file  Load atoms and VMs from a file";
///
/// Plugin API. initializes plugin
///
#[no_mangle] fn init(_core: &Core) {}
///
/// Plugin API. Do main work by haddling terminal commands and call core API
///
#[no_mangle] pub fn idle(core: &Core) {
    match THREAD.with(|rec| rec.try_recv()) {
        Ok(line) => run_command(line.as_str(), core),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("plugin-terminal: Commands channel disconnected"),
    }
}
///
/// Plugin API. Destroys plugin. 
///
#[no_mangle] pub fn remove(_core: &Core) {
    u! { THREAD_STOPPED = true }
}
///
/// Helper function, which create thread and start to listen typed
/// commands into terminal
///
fn create_thread() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        if u! { THREAD_STOPPED } { break }
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
///
/// Handles command typed in a terminal or show unsupported message
///
fn run_command(line: &str, core: &Core) {
    let cmd = line.trim_matches(|c| c == '\r' || c == '\n').split(CMD_SEPARATOR).collect::<Vec<&str>>();
    match cmd[0] {
        "quit" | "q" => core.io.fire(EVENT_QUIT, &Param::None),
        "run"  | "r" => core.io.fire(EVENT_RUN, &Param::None),
        "help" | "h" => println!("{}",HELP_MSG.yellow()),
        "load" | "l" => load_atoms(cmd, core),
        _ => println!("{}", "Unknown command. Type \"help\" for details".red().bold())
    }
}
///
/// Loads atoms and VMs from a file
///
fn load_atoms(cmd: Vec<&str>, core: &Core) {
    if cmd.len() < 2 {
        println!("{}", "File for load wasn't specified. Type \"help\" for details".red().bold());
        return;
    }

    core.io.fire(EVENT_LOAD_DUMP, &Param::LoadAtoms(cmd[1]));
}