//!
//! Part of VM module. Contains atom related stuff.
//!
use crate::global::*;
///
/// Checks if atom is empty. It may be empty if a cell, which is checked is equal to zero
///
pub fn is_atom(atom: Atom) -> bool { atom & ATOM_TYPE_MASK != ATOM_EMPTY }
///
/// Returns atom type (mov, fix, spl,...)
///
pub fn get_type(atom: Atom) -> Atom { (atom & ATOM_TYPE_MASK) >> ATOM_TYPE_SHIFT }
///
/// Returns next atom direction for VM
///
pub fn get_vm_dir(atom: Atom) -> Dir {
    if !has_vm_bond(atom) { return DIR_NO }
    ((atom & ATOM_VM_DIR_MASK) >> ATOM_VM_DIR_SHIFT) as Dir
}
///
/// Sets new atom direction. All other bits keep the same
///
pub fn set_vm_dir(atom: &mut Atom, dir: Dir) {
    *atom = (*atom & ATOM_VM_DIR_UNMASK) | ((dir as Atom) << ATOM_VM_DIR_SHIFT);
    set_vm_bond(atom)
}
///
/// Checks if atom has vm bond
///
pub fn has_vm_bond(atom: Atom) -> bool { (atom & ATOM_VM_BOND_MASK) > 0 }
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
pub fn get_dir1(atom: Atom) -> Dir { ((atom & ATOM_DIR1_MASK) >> ATOM_DIR1_SHIFT) as Dir }
///
/// Sets if atom direction. All other bits keep the same
///
//pub fn set_dir1(atom: &mut Atom, dir: Dir) { *atom = (*atom & ATOM_DIR1_UNMASK) | ((dir as Atom) << ATOM_DIR1_SHIFT) }
///
/// Checks if atom has dir1 bond
///
//pub fn has_dir1_bond(atom: Atom) -> bool { (atom & ATOM_DIR1_BOND_MASK) > 0 }
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
pub fn get_dir2(atom: Atom) -> Dir { ((atom & ATOM_DIR2_MASK) >> ATOM_DIR2_SHIFT) as Dir }
///
/// Sets then atom direction. All other bits keep the same
///
pub fn set_dir2(atom: &mut Atom, dir: Dir) {
    *atom = (*atom & ATOM_DIR2_UNMASK) | ((dir as Atom) << ATOM_DIR2_SHIFT);
    set_dir2_bond(atom)
}
///
/// Checks if atom has dir2 bond
///
pub fn has_dir2_bond(atom: Atom) -> bool { (atom & ATOM_DIR2_BOND_MASK) > 0 }
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

        assert_eq!(atom::is_atom(0b0010_0000_0000_1000), true);
        assert_eq!(atom::is_atom(0b0110_0000_0000_1000), true);

        assert_eq!(atom::is_atom(0b0000_1000_0000_1000), false);
        assert_eq!(atom::is_atom(0b0000_1000_1000_1000), false);
        assert_eq!(atom::is_atom(0b0001_0000_0000_0000), false);
        assert_eq!(atom::is_atom(0b0001_1111_1111_1111), false);
    }
    #[test]
    fn test_get_type() {
        assert_eq!(atom::get_type(0b0000_0000_0000_0000), 0);
        assert_eq!(atom::get_type(0b0001_1111_1111_1111), 0);
        assert_eq!(atom::get_type(0b0010_0000_0000_0000), 1);
        assert_eq!(atom::get_type(0b0110_0000_0000_0000), 0b11);
        assert_eq!(atom::get_type(0b1010_0000_0000_0000), 0b101);
        assert_eq!(atom::get_type(0b1110_0000_0000_0000), 0b111);
        assert_eq!(atom::get_type(0b1011_0000_0000_0000), 0b101);
    }
    #[test]
    fn test_get_vm_dir() {
        assert_eq!(atom::get_vm_dir(0b0000_0000_0000_0000), 0);
        assert_eq!(atom::get_vm_dir(0b0000_0100_0000_0000), 1);
        assert_eq!(atom::get_vm_dir(0b0000_1000_0000_0000), 2);
        assert_eq!(atom::get_vm_dir(0b0001_0000_0000_0000), 4);
        assert_eq!(atom::get_vm_dir(0b1111_0011_1111_1111), 4);
    }
    #[test]
    fn test_set_vm_dir() {
        let mut atom: atom::Atom = 0;
        atom::set_vm_dir(&mut atom, 1);
        assert_eq!(atom::get_vm_dir(atom), 1);

        atom::set_vm_dir(&mut atom, 0);
        assert_eq!(atom::get_vm_dir(atom), 0);

        atom::set_vm_dir(&mut atom, 6);
        assert_eq!(atom::get_vm_dir(atom), 6);

        atom::set_vm_dir(&mut atom, 7);
        assert_eq!(atom::get_vm_dir(atom), 7);

        atom::set_vm_dir(&mut atom, 8);
        assert_eq!(atom::get_vm_dir(atom), 0);
    }
    #[test]
    fn test_has_vm_bond() {
        assert_eq!(atom::has_vm_bond(0b0000_0000_0000_0000), false);
        assert_eq!(atom::has_vm_bond(0b0000_0010_0000_0000), true);
        assert_eq!(atom::has_vm_bond(0b0000_0111_0000_0000), true);
        assert_eq!(atom::has_vm_bond(0b0000_0101_0000_0000), false);
        assert_eq!(atom::has_vm_bond(0b1111_1101_1111_1111), false);
        assert_eq!(atom::has_vm_bond(0b1111_1111_1111_1111), true);
    }
    #[test]
    fn test_set_vm_bond() {
        let mut atom: atom::Atom = 0;
        atom::set_vm_bond(&mut atom);
        assert_eq!(atom::has_vm_bond(atom), true);

        atom::set_vm_bond(&mut atom);
        assert_eq!(atom::has_vm_bond(atom), true);
    }
    #[test]
    fn test_reset_vm_bond() {
        let mut atom: atom::Atom = 0;
        atom::set_vm_bond(&mut atom);
        assert_eq!(atom::has_vm_bond(atom), true);

        atom::reset_vm_bond(&mut atom);
        assert_eq!(atom::has_vm_bond(atom), false);
    }
    #[test]
    fn test_get_dir1() {
        assert_eq!(atom::get_dir1(0b0000_0000_0000_0000), 0);
        assert_eq!(atom::get_dir1(0b0000_0001_1100_0000), 0b111);
        assert_eq!(atom::get_dir1(0b0000_0000_0100_0000), 1);
        assert_eq!(atom::get_dir1(0b0000_0001_0100_0000), 0b101);
        assert_eq!(atom::get_dir1(0b1111_1110_0011_1111), 0);
        assert_eq!(atom::get_dir1(0b1111_1110_1011_1111), 0b10);
    }
    #[test]
    fn test_get_dir2() {
        assert_eq!(atom::get_dir2(0b0000_0000_0000_0000), 0);
        assert_eq!(atom::get_dir2(0b0000_0000_0011_1000), 0b111);
        assert_eq!(atom::get_dir2(0b1111_1111_1100_0111), 0);
        assert_eq!(atom::get_dir2(0b1111_1111_1100_1111), 1);
        assert_eq!(atom::get_dir2(0b1111_1111_1110_1111), 0b101);
        assert_eq!(atom::get_dir2(0b1111_1111_1111_1111), 0b111);
    }
    #[test]
    fn test_set_dir2() {
        let mut atom: atom::Atom = 0;
        atom::set_dir2(&mut atom, 0b111);
        assert_eq!(atom::get_dir2(atom), 0b111);

        atom::set_dir2(&mut atom, 1);
        assert_eq!(atom::get_dir2(atom), 1);

        atom::set_dir2(&mut atom, 0);
        assert_eq!(atom::get_dir2(atom), 0);
    }
    #[test]
    fn test_has_dir2_bond() {
        assert_eq!(atom::has_dir2_bond(0b0000_0000_0000_0000), false);
        assert_eq!(atom::has_dir2_bond(0b0000_0000_0000_0010), true);
        assert_eq!(atom::has_dir2_bond(0b1111_1111_1111_1101), false);
    }
    #[test]
    fn test_set_dir2_bond() {
        let mut atom: atom::Atom = 0;
        atom::set_dir2_bond(&mut atom);
        assert_eq!(atom::has_dir2_bond(atom), true);
    }
    #[test]
    fn test_reset_dir2_bond() {
        let mut atom: atom::Atom = 0;
        atom::set_dir2_bond(&mut atom);
        assert_eq!(atom::has_dir2_bond(atom), true);
        atom::reset_dir2_bond(&mut atom);
        assert_eq!(atom::has_dir2_bond(atom), false);
    }
}