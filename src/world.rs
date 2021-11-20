//!
//! World module. Implements 2d world related stuff.
//! 
use crate::utils;
use crate::global::Atom;
use crate::global::Offs;
use crate::global::ATOM_EMPTY;
///
/// Structure of the world. It consists of cells and atoms inside them
///
pub struct World {
    ///
    /// linear array of dots
    /// 
    cells: Vec<Atom>,
    ///
    /// Max amount of atoms in a world.
    ///
    size: usize
}

impl World {
    ///
    /// Creates new world of atoms
    /// @param len - amount of atoms in a world
    ///
    pub fn new(len: usize) -> Option<World> {
        if len < 1 { return None }
        let mut mem = utils::alloc(len);
        utils::zero(&mut mem, 0);
        Some(
            World {
                cells: mem,
                size: len
            }
        )
    }

    pub fn get_dot(&self, offs: Offs) -> Atom {
        if offs >= self.size { return ATOM_EMPTY }
        self.cells[offs]
    }

    pub fn set_dot(&mut self, offs: Offs, dot: Atom) {
        if offs >= self.size { return }
        self.cells[offs] = dot;
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}