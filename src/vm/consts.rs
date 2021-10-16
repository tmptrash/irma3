//!
//! Module of VM constants. This section contains masks for atom inner bits.
//!
use crate::global::Atom;
///
/// We use 0..1 bits for atom type.
///
pub const ATOM_TYPE_MASK:  Atom = 0b11000000_00000000_00000000_00000000;
///
/// Amount of bits we have to shift righ to get atom type.
/// 
pub const ATOM_TYPE_SHIFT: Atom = 14;
///
/// We use 2..4 bits for VM run direction.
/// 
pub const ATOM_DIR_MASK:   Atom = 0b00111000_00000000_00000000_00000000;
///
/// We use bit 5 as fix/unfix atom type switch.
/// 
pub const ATOM_FIX_MASK:   Atom = 0b00000100_00000000_00000000_00000000;
///
/// We use 2..4 bits for if direction (if atom).
/// 
pub const ATOM_IF_MASK:    Atom = 0b00111000_00000000_00000000_00000000;
///
/// We use 5..7 bits for then direction (if atom).
/// 
pub const ATOM_THEN_MASK:  Atom = 0b00000111_00000000_00000000_00000000;
///
/// We use 8..15 bits for atom connections.
/// 
pub const ATOM_CON_MASK:   Atom = 0b00000000_11111111_00000000_00000000;
