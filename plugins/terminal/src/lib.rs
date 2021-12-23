//!
//! Implementation of terminal plugin. Gives an ability to run terminal commands
//! during app execution.
//!
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread};
use colored::Colorize;
use share::io::IO;
use share::io::Param;
use share::io::events::{*};
//
// Local variable of this module, which keeps thread instance
//
thread_local!(static THREAD: Receiver<String> = create_thread());
///
/// Local variable, which affects thread activeness
///
static mut THREAD_STOPPED: bool = false;
///
/// Plugin API. initializes plugin
///
#[no_mangle] fn init(_io: &IO) {}
///
/// Plugin API. Do main work by haddling terminal commands and call core API
///
#[no_mangle] pub fn idle(io: &IO) {
    match THREAD.with(|rec| rec.try_recv()) {
        Ok(key) => run_command(key.as_str(), io),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("plugin-terminal: Commands channel disconnected"),
    }
}
///
/// Plugin API. Destroys plugin. 
///
#[no_mangle] pub fn remove(_io: &IO) {
    unsafe { THREAD_STOPPED = true }
}
///
/// Helper function, which create thread and start to listen typed
/// commands into terminal
///
fn create_thread() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        if unsafe { THREAD_STOPPED } { break }
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
///
/// Handles command typed in a terminal or show unsupported message
///
fn run_command(cmd: &str, io: &IO) {
    match cmd.trim_matches(|c| c == '\r' || c == '\n') {
        "quit" | "q" => io.fire(EVENT_QUIT, &Param::None),
        "run"  | "r" => io.fire(EVENT_RUN, &Param::None),
        "help" | "h" => println!("{}",
"Supported commands:
    q, quit   Quit the system
    r, run    Run or stop the system
    h, help   Show this message".yellow()),
        _      => { println!("{}", "Unknown command. Type \"help\" for details".red()) }
    }
}