//!
//!  Application wide definitions.
//!
///
/// One atom type. We use 4 bytes atom to store type, VM direction bounds,
/// and atom specific bits.
/// 
pub type Atom = u32;
///
/// One of 8 possible directions (0..=7)
///
pub type Dir = u8;
///
/// Offset in a world
///
pub type Offs = usize;
///
/// Empty atom. Means that current cell is empty
///
pub const ATOM_EMPTY:         Atom = 0;
///
/// We use 0..1 bits for atom type.
///
pub const ATOM_TYPE_MASK:     Atom = 0b11100000_00000000_00000000_00000000;
///
/// Amount of bits we have to shift righ to get atom type.
/// 
pub const ATOM_TYPE_SHIFT:    Atom = 29;
///
/// We use 2..4 bits for VM run direction.
/// 
pub const ATOM_DIR_MASK:      Atom = 0b00011100_00000000_00000000_00000000;
///
/// Amount of bits we have to shift righ to get atom direction.
/// 
pub const ATOM_DIR_SHIFT:     Atom = 26;
///
/// We use bit 5 as fix/unfix atom type switch.
/// 
pub const ATOM_FIX_MASK:      Atom = 0b00000100_00000000_00000000_00000000;
///
/// We use 2..4 bits for if direction (if atom).
/// 
pub const ATOM_IF_MASK:       Atom = 0b00111000_00000000_00000000_00000000;
///
/// We use 5..7 bits for then direction (if atom).
/// 
pub const ATOM_THEN_MASK:     Atom = 0b00000111_00000000_00000000_00000000;
///
/// We use 8..15 bits for atom connections.
/// 
pub const ATOM_CON_MASK:      Atom = 0b00000000_11111111_00000000_00000000;
///
/// We use 6..8 bits for direction in mov atom type
///
pub const ATOM_MOV_DIR:       Atom = 0b00000011_10000000_00000000_00000000;
///
/// Amount of bits we have to shift to get mov atom direction
///
pub const ATOM_MOV_DIR_SHIFT: Atom = 23;