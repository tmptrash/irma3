//!
//! Events list, which is used in IO module. All these constants should be 
//! with increasing index. EVENT_LAST should be the last index in a vector.
//!
///
/// Draws a dot in a world
/// 
pub const EVENT_SET_DOT: usize = 0;
///
/// Runs or pause the system
///
pub const EVENT_RUN:     usize = 1;
///
/// Stops the system and exits
///
pub const EVENT_QUIT:    usize = 2;
///
/// Should be a last event. Every time you add new event in this list,
/// please update this number
///
pub const EVENT_LAST:    usize = 3;