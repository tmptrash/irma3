//!
//! Part of VM module. Contains atom related stuff.
//!
use crate::global::*;
///
/// Checks if atom is empty (just empty world's cell) or not
///
pub fn is_atom(atom: Atom) -> bool { atom & ATOM_TYPE_MASK != ATOM_EMPTY }
///
/// Returns atom type
///
pub fn get_type(atom: Atom) -> Atom { atom & ATOM_TYPE_MASK >> ATOM_TYPE_SHIFT }
///
/// Returns next atom direction for VM
///
pub fn get_vm_dir(atom: Atom) -> Dir { (atom & ATOM_VM_DIR_MASK >> ATOM_VM_DIR_SHIFT) as Dir }
///
/// Sets new atom direction. All other bits keep the same
///
pub fn set_vm_dir(atom: &mut Atom, dir: Dir) { *atom = (*atom & ATOM_VM_DIR_UNMASK) | ((dir as Atom) << ATOM_VM_DIR_SHIFT) }
///
/// Checks if atom has vm bond
///
pub fn has_vm_bond(atom: Atom) -> bool { atom & ATOM_VM_BOND_MASK > 0 }
///
/// Sets VM bond bit (set to 1)
///
pub fn set_vm_bond(atom: &mut Atom) { *atom |= ATOM_VM_BOND_MASK }
///
/// Clears VM bond bit (set to 0)
///
pub fn reset_vm_bond(atom: &mut Atom) { *atom &= ATOM_VM_BOND_UNMASK }
///
/// Returns if atom direction
///
pub fn get_dir1(atom: Atom) -> Dir { (atom & ATOM_DIR1_MASK >> ATOM_DIR1_SHIFT) as Dir }
///
/// Sets if atom direction. All other bits keep the same
///
//pub fn set_dir1(atom: &mut Atom, dir: Dir) { *atom = (*atom & ATOM_DIR1_UNMASK) | ((dir as Atom) << ATOM_DIR1_SHIFT) }
///
/// Checks if atom has dir1 bond
///
//pub fn has_dir1_bond(atom: Atom) -> bool { atom & ATOM_DIR1_BOND_MASK > 0 }
///
/// Sets dir1 bond bit (set to 1)
///
//pub fn set_dir1_bond(atom: &mut Atom) { *atom |= ATOM_DIR1_BOND_MASK }
///
/// Clears dir1 bond bit (set to 0)
///
//pub fn reset_dir1_bond(atom: &mut Atom) { *atom &= ATOM_DIR1_BOND_UNMASK }
///
/// Returns then atom direction
///
pub fn get_dir2(atom: Atom) -> Dir { (atom & ATOM_DIR2_MASK >> ATOM_DIR2_SHIFT) as Dir }
///
/// Sets then atom direction. All other bits keep the same
///
pub fn set_dir2(atom: &mut Atom, dir: Dir) { *atom = (*atom & ATOM_DIR2_UNMASK) | ((dir as Atom) << ATOM_DIR2_SHIFT) }
///
/// Checks if atom has dir2 bond
///
pub fn has_dir2_bond(atom: Atom) -> bool { atom & ATOM_DIR2_BOND_MASK > 0 }
///
/// Sets dir2 bond bit (set to 1)
///
pub fn set_dir2_bond(atom: &mut Atom) { *atom |= ATOM_DIR2_BOND_MASK }
///
/// Clears dir2 bond bit (set to 0)
///
pub fn reset_dir2_bond(atom: &mut Atom) { *atom &= ATOM_DIR2_BOND_UNMASK }


#[cfg(test)]
mod tests {
    use crate::atom;

    #[test]
    fn test_is_atom() {
        assert_eq!(atom::is_atom(0b0000_0000_0000_0000), false);
        assert_eq!(atom::is_atom(0b0010_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b0100_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b0110_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b1000_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b1010_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b1100_0000_0000_0000), true);
        assert_eq!(atom::is_atom(0b1110_0000_0000_0000), true);
    }
}