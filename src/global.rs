//!
//!  Application wide definitions.
//!
///
/// One atom type. We use 4 bytes atom to store type, VM direction bounds,
/// and atom specific bits.
/// 
pub type Atom = u32;