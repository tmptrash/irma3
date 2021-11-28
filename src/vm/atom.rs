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
pub fn set_vm_dir(atom: Atom, dir: Dir) -> Atom {
    (atom & ATOM_DIR_UNMASK) | ((dir as Atom) << ATOM_DIR_SHIFT)
}