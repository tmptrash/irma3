use std::mem;
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
        let bytes = mem::size_of::<Atom>() * len;
        let mut v = Vec::with_capacity(bytes);
        unsafe { v.set_len(bytes) }

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