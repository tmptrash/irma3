//!
//! Application wide definitions. Here are definitions, which may be used 
//! in every module
//!
///
/// Alias for usize to have short version of [i as I] instead [i as usize]
/// 
pub type I = usize;
///
/// One atom type. We use 2 bytes atom to store type, VM move direction (bound),
/// and atom type specific bits.
///
pub type Atom = u16;
///
/// One of 8 possible directions (0..=7)
///
pub type Dir = u8;
///
/// Atom bonds type. Every bit mean one bone. Bit 0 - up, 1 - up, right,...
///
pub type Bonds = u8;
///
/// Offset in a world
///
pub type Offs = isize;
///
/// Means that atom was run normally
///
pub const RET_OK: u8 = 0;
///
/// Means thar atom was not run
///
pub const RET_SKIPPED: u8 = 1;
///
/// Means that job atom was run
///
pub const RET_ADD_VM: u8 = 2;
///
/// Amount of possible directions
///
pub const DIRS_LEN: usize = 8;
///
/// Means no direction
///
pub const DIR_NO: Dir = Dir::MAX;
///
/// Empty atom. Means that current cell is empty
///
pub const ATOM_EMPTY:          Atom = 0;
///
/// We use 0..1 bits for atom type.
///
pub const ATOM_TYPE_MASK:      Atom = 0b11100000_00000000;
///
/// Amount of bits we have to shift righ to get atom type.
///
pub const ATOM_TYPE_SHIFT:     Atom = 13;
///
/// We use 2..4 bits for VM run direction.
///
pub const ATOM_VM_DIR_MASK:    Atom = 0b00011100_00000000;
///
/// Mask to reset direction bits
///
pub const ATOM_VM_DIR_UNMASK:    Atom = 0b11100011_11111111;
///
/// Amount of bits we have to shift righ to get atom direction.
///
pub const ATOM_VM_DIR_SHIFT:     Atom = 10;
///
/// A bit of VM bond (0 - no bond, has a bond)
///
pub const ATOM_VM_BOND_MASK:     Atom = 0b00000010_00000000;
///
/// A bit of VM bond (0 - no bond, has a bond)
///
pub const ATOM_VM_BOND_UNMASK:   Atom = 0b11111101_11111111;
///
/// We use 6..8 bits for if direction (if atom).
///
pub const ATOM_DIR1_MASK:        Atom = 0b00000001_11000000;
///
/// Amount of bits we have to shift righ to get atom if direction.
///
pub const ATOM_DIR1_SHIFT:       Atom = 6;
///
/// Mask to reset if direction bits
///
pub const ATOM_DIR1_UNMASK:      Atom = 0b11111110_00111111;
///
/// A bit of dir1 bond (0 - no bond, has a bond)
///
pub const ATOM_DIR1_BOND_MASK:   Atom = 0b00000000_00000100;
///
/// A bit of dir1 bond (0 - no bond, has a bond)
///
pub const ATOM_DIR1_BOND_UNMASK: Atom = 0b11111111_11111011;
///
/// We use 6..8 bits for if direction (if atom).
///
pub const ATOM_DIR2_MASK:        Atom = 0b00000000_00111000;
///
/// Amount of bits we have to shift righ to get atom then direction.
///
pub const ATOM_DIR2_SHIFT:       Atom = 3;
///
/// Mask to reset then direction bits
///
pub const ATOM_DIR2_UNMASK:      Atom = 0b11111111_11000111;
///
/// A bit of dir2 bond (0 - no bond, has a bond)
///
pub const ATOM_DIR2_BOND_MASK:   Atom = 0b00000000_00000010;
///
/// A bit of dir2 bond (0 - no bond, has a bond)
///
pub const ATOM_DIR2_BOND_UNMASK: Atom = 0b11111111_11111101;
///
/// We use 6..8 bits for direction in mov atom type
///
pub const ATOM_MOV_DIR:          Atom = 0b00000001_11000000;
///
/// Amount of bits we have to shift to get mov atom direction
///
pub const ATOM_MOV_DIR_SHIFT:    Atom = 6;
///
/// Reverted directions. Are used in a process of update atom 
/// bonds during atom moving.
/// 4 5 6
/// 3 X 7
/// 2 1 0
/// 
pub const DIR_REV: [Dir; DIRS_LEN] = [4, 5, 6, 7, 0, 1, 2, 3];
///
/// Directions map for the atom, which is moving. Is used for 
/// updating it's bonds
///
pub const DIR_MOV_ATOM: [[Dir; DIRS_LEN]; DIRS_LEN] = [
    [DIR_NO,      7, DIR_NO, DIR_NO, DIR_NO, DIR_NO, DIR_NO,      1],
    [     3, DIR_NO,      7,      0, DIR_NO, DIR_NO, DIR_NO,      2],
    [DIR_NO,      3, DIR_NO,      1, DIR_NO, DIR_NO, DIR_NO, DIR_NO],
    [DIR_NO,      4,      5, DIR_NO,      1,      2, DIR_NO, DIR_NO],
    [DIR_NO, DIR_NO, DIR_NO,      5, DIR_NO,      3, DIR_NO, DIR_NO],
    [DIR_NO, DIR_NO, DIR_NO,      6,      7, DIR_NO,      3,      4],
    [DIR_NO, DIR_NO, DIR_NO, DIR_NO, DIR_NO,      7, DIR_NO,      5],
    [     5,      6, DIR_NO, DIR_NO, DIR_NO,      0,      1, DIR_NO]
];
///
/// Directions map for the atom, which is near the moved atom. Is used for 
/// updating it's (near) bonds
///
pub const DIR_NEAR_ATOM: [[Dir; DIRS_LEN]; DIRS_LEN] = [
    [DIR_NO, DIR_NO, DIR_NO,      1, DIR_NO,      7, DIR_NO, DIR_NO],
    [DIR_NO, DIR_NO, DIR_NO,      2,      3, DIR_NO,      7,      0],
    [DIR_NO, DIR_NO, DIR_NO, DIR_NO, DIR_NO,      3, DIR_NO,      1],
    [     1,      2, DIR_NO, DIR_NO, DIR_NO,      4,      5, DIR_NO],
    [DIR_NO,      3, DIR_NO, DIR_NO, DIR_NO, DIR_NO, DIR_NO,      5],
    [     7, DIR_NO,      3,      4, DIR_NO, DIR_NO, DIR_NO,      6],
    [DIR_NO,      7, DIR_NO,      5, DIR_NO, DIR_NO, DIR_NO, DIR_NO],
    [DIR_NO,      0,      1, DIR_NO,      5,      6, DIR_NO, DIR_NO]
];