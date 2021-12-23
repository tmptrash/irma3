//!
//! IO module. Connects core (world, VMs and main loop) with plugins
//!
pub mod events;

use crate::global::Offs;
use crate::global::Atom;
use events::{*};
///
/// Shorthand for callback function
///
pub type Callback = fn(&Param);
///
/// Enum for different event parameters types
///
#[derive(Clone, Copy)]
pub enum Param {
    None,                                                       // No parameters
    SetDot(Offs, Atom),                                         // Draw atom by offset
    Run(bool),                                                  // Runs or stops a s
    Stop                                                        // Stop an app
}
///
/// Event bus object. Holds all listeners by event
///
pub struct IO {
    events: Vec<Vec<Callback>>
}

impl IO {
    pub fn new() -> IO {
        let mut io = IO { events: Vec::new() };
        for _i in 0..EVENT_LAST { io.events.push(Vec::new()) }
        io
    }
    ///
    /// Assigns listener (callback function) to event and return listener id,
    /// which is used in off() function
    ///
    pub fn on(&mut self, event: usize, cb: Callback) -> usize {
        self.events[event].push(cb);
        self.events[event].len() - 1 // listener id
    }
    ///
    /// Unassigns listener (callback function) from event by listener id
    ///
    pub fn off(&mut self, event: usize, listener_id: usize) {
        self.events[event].remove(listener_id);
    }
    ///
    /// Fires an event with parameter
    ///
    pub fn fire(&self, event: usize, p: &Param) {
        for cb in &self.events[event] { cb(p) }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Param;
    use crate::io::IO;
    use crate::io::events::{*};
    static mut BOOL_VAR: bool = false;

    #[test]
    fn test_new() {
        let mut io = IO::new();
        io.on(EVENT_RUN, |p: &Param| { unsafe { if let Param::Run(r) = p { BOOL_VAR = *r }} });
        io.fire(EVENT_RUN, &Param::Run(true));
        assert_eq!(unsafe { BOOL_VAR }, true);
    }
}