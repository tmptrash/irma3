//!
//! Module of VM related data. There should be only one instance of VMData
//!
use crate::world::World;
use crate::vm::buf::MoveBuffer;
use crate::cfg::AtomConfig;
use share::global::Dir;
use share::global::DIRS_LEN;
use share::io::IO;
///
/// Data needed for VM to work. Should be set from outside of VM
///
pub struct VMData<'a, 'b> {
    ///
    /// Reference to the world data
    ///
    pub world: World,
    ///
    /// Shared between VMs buffer. Is used in mov atom.
    ///
    pub buf: MoveBuffer,
    ///
    /// Reverted directions, which is used in mov atom
    ///
    pub dirs_rev: [Dir; DIRS_LEN],
    ///
    /// Atoms related configuration
    ///
    pub atoms_cfg: &'a AtomConfig,
    ///
    /// Reference to IO mechanism
    ///
    pub io: &'b IO
}