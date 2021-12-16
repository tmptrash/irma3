//!
//! Implementation of terminal plugin. Gives an ability to type command line commands
//! during app execution
//!
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread};
use crate::io::IO;
//
// Local variable of this module, which keeps thread object
//
thread_local!(static THREAD: Receiver<String> = create_thread());
///
/// Module local variable, which affects thread activeness
///
static mut STOPPED: bool = false;

pub fn init(_io: &IO) {}

pub fn idle(io: &IO) {
    match THREAD.with(|rec| rec.try_recv()) {
        Ok(key) => print!("Received: {}", key),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
}

pub fn destroy(io: &IO) {
    unsafe { STOPPED = true }
}

fn create_thread() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        if unsafe { STOPPED } { break }
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}