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
/// Empty callback
///
fn empty_fn(_: &Param) {}
///
/// Enum for different event parameters types
///
pub enum Param<'a> {
    None,                                                       // No parameters
    SetDot(Offs, Atom),                                         // Draw an atom by offset
    MoveDot(Offs, Offs, Atom),                                  // Moves an atom from offs0 to offs1
    LoadAtoms(&'a str),                                         // Loads atoms and VMs from a file
    SaveAtoms(&'a str)                                          // Saves atoms and VMs to a file
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
    /// Unassigns listener (callback function) from event by listener id.
    /// It's important to only mark unused listeners as "empty" because we use 
    /// indexes as listener id to prevent change all other ids
    ///
    pub fn off(&mut self, event: usize, listener_id: usize) {
        self.events[event][listener_id] = empty_fn;
    }
    ///
    /// Fires an event with parameter
    ///
    pub fn fire(&self, event: usize, p: &Param) {
        for cb in &self.events[event] {
            if *cb as usize != empty_fn as usize { cb(p) }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Param;
    use crate::io::IO;
    use crate::io::events::{*};
    static mut BOOL_VAR: bool = false;
    static mut BOOL_VAR1: bool = false;

    #[test]
    fn test_new() {
        let mut io = IO::new();
        io.on(EVENT_RUN, |_p| { unsafe { BOOL_VAR = true } });
        io.fire(EVENT_RUN, &Param::None);
        assert_eq!(unsafe { BOOL_VAR }, true);

        unsafe { BOOL_VAR = false; BOOL_VAR1 = false };
    }
    #[test]
    fn test_on_fire() {
        let mut io = IO::new();
        io.on(EVENT_SAVE_DUMP, |p| { unsafe { BOOL_VAR = if let Param::SaveAtoms(file) = p { *file == "file" } else { false }  } });
        io.fire(EVENT_SAVE_DUMP, &Param::SaveAtoms("file"));
        assert_eq!(unsafe { BOOL_VAR }, true);

        unsafe { BOOL_VAR = false; BOOL_VAR1 = false };
    }
    #[test]
    fn test_fire_should_not_affect_other_handlers() {
        let mut io = IO::new();
        io.on(EVENT_SAVE_DUMP, |_| { unsafe { BOOL_VAR = true }});
        io.on(EVENT_SET_DOT, |_| { unsafe { BOOL_VAR1 = true }});
        io.on(EVENT_MOVE_DOT, |_| { unsafe { BOOL_VAR1 = true }});
        io.on(EVENT_RUN, |_| { unsafe { BOOL_VAR1 = true }});
        io.on(EVENT_QUIT, |_| { unsafe { BOOL_VAR1 = true }});
        io.on(EVENT_LOAD_DUMP, |_| { unsafe { BOOL_VAR1 = true }});

        io.fire(EVENT_SAVE_DUMP, &Param::SaveAtoms("data"));
        assert_eq!(unsafe { BOOL_VAR && !BOOL_VAR1 }, true);

        unsafe { BOOL_VAR = false; BOOL_VAR1 = false };
    }
    #[test]
    fn test_off() {
        let mut io = IO::new();
        let id = io.on(EVENT_RUN, |_| { unsafe { BOOL_VAR = true }});
        io.fire(EVENT_RUN, &Param::None);
        assert_eq!(unsafe { BOOL_VAR }, true);
        unsafe { BOOL_VAR = false; BOOL_VAR1 = false };

        io.off(EVENT_RUN, id);
        io.fire(EVENT_RUN, &Param::None);
        assert_eq!(unsafe { !BOOL_VAR && !BOOL_VAR1 }, true);

        unsafe { BOOL_VAR = false; BOOL_VAR1 = false };
    }
}