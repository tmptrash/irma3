//!
//! Part of VM module. Contains atom related stuff.
//! 
use crate::global::{*};
///
/// Checks if atom is empty (just empty world's cell) or not
///
pub fn is_atom(atom: Atom) -> bool { atom & ATOM_TYPE_MASK >> ATOM_TYPE_SHIFT == ATOM_EMPTY }
///
/// Returns atom type
///
pub fn get_type(atom: Atom) -> Atom { atom & ATOM_TYPE_MASK >> ATOM_TYPE_SHIFT }
///
/// Returns next atom direction for VM
///
pub fn get_vm_dir(atom: Atom) -> Dir { (atom & ATOM_DIR_MASK >> ATOM_DIR_SHIFT) as Dir }
///
/// Sets new atom direction. All other bits keep the same
///
pub fn set_vm_dir(atom: Atom, dir: Dir) -> Atom { (atom & ATOM_DIR_UNMASK) | ((dir as Atom) << ATOM_DIR_SHIFT) }
///
/// Returns if atom direction
///
pub fn get_if_dir(atom: Atom) -> Dir { (atom & ATOM_IF_MASK >> ATOM_IF_SHIFT) as Dir }
///
/// Sets if atom direction. All other bits keep the same
///
pub fn set_if_dir(atom: Atom, dir: Dir) -> Atom { (atom & ATOM_IF_UNMASK) | ((dir as Atom) << ATOM_IF_SHIFT) }
///
/// Returns then atom direction
///
pub fn get_then_dir(atom: Atom) -> Dir { (atom & ATOM_IF_MASK >> ATOM_IF_SHIFT) as Dir }
///
/// Sets then atom direction. All other bits keep the same
///
pub fn set_then_dir(atom: Atom, dir: Dir) -> Atom { (atom & ATOM_IF_UNMASK) | ((dir as Atom) << ATOM_IF_SHIFT) }