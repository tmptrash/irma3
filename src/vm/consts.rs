//
// Module of VM constants.
// This section contains masks for atom inner bits
//
pub const ATOM_TYPE_MASK: u16 = 0b11000000_00000000; // We use 0..1 bits for atom type
pub const ATOM_DIR_MASK:  u16 = 0b00111000_00000000; // We use 2..4 bits for VM run direction
pub const ATOM_FIX_MASK:  u16 = 0b00000100_00000000; // We use bit 5 as fix/unfix atom type switch
pub const ATOM_IF_MASK:   u16 = 0b00111000_00000000; // We use 2..4 bits for if direction (if atom)
pub const ATOM_THEN_MASK: u16 = 0b00000111_00000000; // We use 5..7 bits for then direction (if atom)
pub const ATOM_CON_MASK:  u16 = 0b00000000_11111111; // We use 8..15 bits for atom connections