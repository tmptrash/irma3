//!
//! IO module. Connects core (world, VMs and main loop) with plugins
//!
pub mod events;

use crate::global::Offs;
use crate::global::Atom;
use crate::cfg::Config;
///
/// Shorthand for callback function
///
pub type Callback = fn(&Params);
///
/// Describes parameters of IO events
///
pub struct Params<'a> {
    pub param: Param,
    pub cfg: &'a Config
}
///
/// Enum for different event parameters types
///
pub enum Param {
    None,                                                       // No parameters
    SetDot(Offs, Atom),                                         // Draw an atom by offset
    MoveDot(Offs, Offs, Atom)                                   // Moves an atom from offs0 to offs1
}
///
/// Event bus object. Holds all listeners by event
///
pub struct IO<'a> {
    events: Vec<Vec<Callback>>,
    pub cfg: &'a Config
}

impl<'a> IO<'a> {
    pub fn new(events: usize, cfg: &Config) -> IO {
        let mut io = IO { events: Vec::new(), cfg };
        for _i in 0..events { io.events.push(Vec::new()) }
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
    pub fn fire(&self, event: usize, p: &Params) {
        for cb in &self.events[event] { cb(p) }
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::Config;
    use crate::io::Param;
    use crate::io::Params;
    use crate::io::IO;
    use crate::io::events::{*};
    static mut BOOL_VAR: bool = false;

    #[test]
    fn test_new() {
        let cfg = Config::new();
        let mut io = IO::new(EVENT_LAST, &cfg);
        io.on(EVENT_RUN, |_p| { unsafe { BOOL_VAR = true } });
        io.fire(EVENT_RUN, &Params {param: Param::None, cfg: &cfg});
        assert_eq!(unsafe { BOOL_VAR }, true);
    }
}