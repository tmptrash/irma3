use std::mem;
use crate::utils;
//
//  One atom type. We use 2 bytes atom to store type, bounds, 
//
pub type Atom = u16;
//
// Structure of the world. It consists of cells and atoms inside them
//
pub struct World {
    cells: Vec<Atom> // linear array of dots
}

impl World {
    //
    // Creates new world of atoms
    // @param len - amount of atoms in a world
    //
    pub fn new(len: usize) -> World {
        World {
            cells: utils::alloc(len, 0)
        }
    }

    pub fn get_dot(&self, offs: usize) -> Atom {
        self.cells[offs]
    }

    pub fn set_dot(&mut self, offs: usize, dot: Atom) {
        self.cells[offs] = dot;
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}