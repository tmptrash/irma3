//!
//! Events list, which is used in IO module. All these constants should be 
//! with increasing index. EVENT_LAST should be the last index in a vector.
//!
///
/// Draws a dot in a world
/// 
pub const EVENT_SET_DOT:    usize = 0;
///
/// Moves a dot in a world from offs0 into offs1
/// 
pub const EVENT_MOVE_DOT:   usize = 1;
///
/// Runs or pause the system
///
pub const EVENT_RUN:        usize = 2;
///
/// Stops the system and exits
///
pub const EVENT_QUIT:       usize = 3;
///
/// Loads atoms and VMs into the world from a dump file
///
pub const EVENT_LOAD_DUMP:  usize = 4;
///
/// Saves atoms and VMs into the world from a dump file
///
pub const EVENT_SAVE_DUMP:  usize = 5;
///
/// Should be a last event. Every time you add new event in this list,
/// please update this number
///
pub const EVENT_LAST:       usize = 6;