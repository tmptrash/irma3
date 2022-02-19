//!
//! IO module. Connects core (world, VMs and main loop) with plugins
//!
pub mod events;

use log::{*};
use crate::global::Offs;
use crate::global::Atom;
use crate::inf;
use crate::sec;

use self::events::EVENT_LAST;
///
/// Shorthand for callback function
///
pub type Callback = fn(&Param);
///
/// Enum for different event parameters types
///
pub enum Param<'a> {
    None,                                                       // No parameters
    SetDot(Offs, Atom),                                         // Draw an atom by offset
    MoveDot(Offs, Offs, Atom),                                  // Moves an atom from offs0 to offs1
    LoadAtoms(&'a String)                                       // Loads atoms and VMs from a file
}
///
/// Event bus object. Holds all listeners by event
///
pub struct IO {
    events: Vec<Vec<Callback>>
}

impl IO {
    pub fn new() -> IO {
        sec!("Create IO object");
        let mut io = IO { events: Vec::new() };
        for _i in 0..EVENT_LAST { io.events.push(Vec::new()) }
        inf!("IO supports {} events", EVENT_LAST);

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
    // pub fn off(&mut self, event: usize, listener_id: usize) {
    //     self.events[event].remove(listener_id);
    // }
    ///
    /// Fires an event with parameter
    ///
    pub fn fire(&self, event: usize, p: &Param) {
        for cb in &self.events[event] { cb(p) }
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::Config;
    use crate::io::Param;
    use crate::io::IO;
    use crate::io::events::{*};
    static mut BOOL_VAR: bool = false;

    #[test]
    fn test_new() {
        let cfg = Config::new();
        let mut io = IO::new();
        io.on(EVENT_RUN, |_p| { unsafe { BOOL_VAR = true } });
        io.fire(EVENT_RUN, &Param::None);
        assert_eq!(unsafe { BOOL_VAR }, true);
    }
}