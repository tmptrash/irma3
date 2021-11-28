//!
//! World module. Implements 2d world related stuff. Difference between
//! this module and atom.rs is that this module manages dots in a world
//! and know nothing about atoms and their inner structure.
//! 
use crate::utils;
use crate::global::Atom;
use crate::global::Offs;
use crate::global::Dir;
use crate::global::ATOM_EMPTY;
use crate::global::DIRS_LEN;
use crate::global::I;
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
    size: usize,
    ///
    /// All possible directions of nearest atoms
    ///
    pub dirs: [i32; DIRS_LEN]
}

impl World {
    ///
    /// Creates new world of atoms
    /// @param len - amount of atoms in a world
    ///
    pub fn new(width: usize, height: usize, dirs: [i32; 8]) -> Option<World> {
        let size = width * height;
        if size < 1 { return None }
        let mut mem = utils::alloc(size);
        utils::zero(&mut mem, 0);
        Some(
            World {
                cells: mem,
                width: width,
                height: height,
                size: size,
                dirs: dirs
            }
        )
    }
    ///
    /// Checks if two dots in a world are near each other. Near means
    /// not more than 1 dot. e.g.: xx - near, x x - not
    ///
    pub fn is_near(offs0: Offs, offs1: Offs, width: usize) -> bool {
        let distance = ((offs0 - offs1) as i32).abs();
        if distance < 2 { return true }
        if distance / width as i32 > 1 { return false }
        (distance % width as i32).abs() < 2
    }
    ///
    /// Returns direction between two dots in a world. The direction is
    /// obtained from the perspective of first dot. This function assumes
    /// that two offsets are near each other (distance == 1) and within
    /// the world
    ///
    pub fn get_dir(offs0: Offs, offs1: Offs) -> Dir {
        // 0, 1, 2
        if offs1 < offs0 - 1 {}
        // 4, 5, 6
        else if offs1 > offs0 + 1 {}
        else if offs0 - 1 == offs1 { return 7 }
        3
    }
    ///
    /// Returns new offset by start offset and direction
    ///
    pub fn get_offs(&self, offs: Offs, dir: Dir) -> Offs {
        let offs = offs + self.dirs[dir as I] as Offs;
        if offs < 0 { return self.size as Offs - 1 }
        else if offs >= self.size as Offs { return 0 }
        offs
    }

    pub fn get_dot(&self, offs: Offs) -> Atom {
        if offs >= self.size as Offs { return ATOM_EMPTY }
        self.cells[offs as I]
    }

    pub fn set_dot(&mut self, offs: Offs, dot: Atom) {
        if offs >= self.size as Offs { return }
        self.cells[offs as I] = dot;
    }

    pub fn mov_dot(&mut self, src_offs: Offs, dest_offs: Offs, dot: Atom) {
        if dest_offs >= self.size as Offs { return }
        self.cells[dest_offs as I] = dot;
        self.cells[src_offs as I] = ATOM_EMPTY;
    }
}