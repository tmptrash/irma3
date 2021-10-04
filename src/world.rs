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
    pub fn new(len: usize) -> World {
        let mut v = utils::alloc(mem::size_of::<Atom>() * len);
        //
        // This peace of code init memory and allocate it, because
        // utils::alloc() doesn't really reserve the memory
        //
        for i in 0..len {
            v[i] = 0 as Atom;
        }
        World {
            cells: v
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