//!
//! Implementation of terminal plugin. Gives an ability to run terminal commands
//! during app execution.
//!
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread};
use share::io::IO;
use share::io::Param;
use share::io::events::{*};
//
// Local variable of this module, which keeps thread instance
//
thread_local!(static THREAD: Receiver<String> = create_thread());
///
/// Module local variable, which affects thread activeness
///
static mut THREAD_STOPPED: bool = false;
///
/// Runs or pauses the system
///
static mut PAUSED: bool = true;
///
/// Plugin API. initializes plugin
///
#[no_mangle] fn init(io: &IO) {}
///
/// Plugin API. Do main work by haddling terminal commands and call core API
///
#[no_mangle] pub fn idle(io: &IO) {
    match THREAD.with(|rec| rec.try_recv()) {
        Ok(key) => match key.as_str().trim_matches(|c| c == '\r' || c == '\n') {
            "quit" | "q" => io.fire(EVENT_EXIT, &Param::None),
            "run"  | "r" => {
                unsafe { PAUSED = !PAUSED };
                io.fire(EVENT_RUN, &Param::Run(unsafe { PAUSED }));
            },
            _      => { println!("Unknown command") }
        },
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