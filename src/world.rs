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
    /// World width
    ///
    pub width: usize,
    ///
    /// World height
    ///
    pub height: usize,
    ///
    /// linear array of dots
    /// 
    cells: Vec<Atom>,
    ///
    /// World size (width * height)
    ///
    size: usize
}

impl World {
    ///
    /// Creates new world of atoms
    /// @param len - amount of atoms in a world
    ///
    pub fn new(width: usize, height: usize) -> Option<World> {
        let size = width * height;
        if size < 1 { return None }
        let mut mem = utils::alloc(size);
        utils::zero(&mut mem, 0);
        Some(
            World {
                cells: mem,
                width: width,
                height: height,
                size: size
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

    pub fn size(&self) -> usize {
        self.cells.len()
    }
    ///
    /// Checks if two dots in a world are near each other. Near means
    /// not more than 1 dot. e.g.: xx - near, x x - not
    ///
    pub fn is_near(offs0: Offs, offs1: Offs, width: usize) -> bool {
        let distance = ((offs0 - offs1) as i32).abs();
        if distance < 2 { return true }
        if distance / width as i32 > 1 { return false }
        (distance % width as i32) < 2
    }
}