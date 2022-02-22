//!
//! Module of definitions for the core
//! 
use std::ffi::c_void;
///
/// Global reference to Core struct. Is used for access to it from core and
/// all available plugins
///
pub static mut CORE: *mut c_void = 0 as *mut c_void;
///
/// Macro to simplify global access to the Core
///
macro_rules! core { () => { u!{ &mut *(crate::defs::CORE as *mut Core) }}}
///
/// Macro to simplify global access to Vector<VM>
///
macro_rules! vms { () => { u!{ &mut (*(crate::defs::CORE as *mut Core)).vms }}}
///
/// Macro to simplify global access to Config
///
macro_rules! cfg { () => { u!{ &mut (*(crate::defs::CORE as *mut Core)).cfg }}}
///
/// Macro to simplify global access to IO
///
macro_rules! io  { () => { u!{ &mut (*(crate::defs::CORE as *mut Core)).io }}}
///
/// Macro to simplify global access to IO
///
macro_rules! vm_data { () => { u!{ &mut (*(crate::defs::CORE as *mut Core)).vm_data }}}