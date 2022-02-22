//!
//! Module of VM related data. There should be only one instance of VMData
//!
use log::*;
use crate::global::{DIR_REV, DIRS_LEN};
use crate::world::World;
use crate::vm::buf::MoveBuffer;
use crate::global::Dir;
use crate::sec;
///
/// Data needed for VM to work. Should be set from outside of VM
///
pub struct VMData {
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
    pub dirs_rev: [Dir; DIRS_LEN]
}

impl VMData {
    ///
    /// Creates VMData struct instance
    ///
    pub fn new(w: usize, h: usize, dir2offs: [i32; 8], mov_buf_size: usize) -> Self {
        sec!("Create shared VM data");
        VMData {
            world: World::new(w, h, dir2offs),
            buf: MoveBuffer::new(mov_buf_size),
            dirs_rev: DIR_REV
        }
    }
}