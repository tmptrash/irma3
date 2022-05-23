//!
//! World module. Implements 2d world related stuff. Difference between
//! this module and atom.rs is that this module manages dots in a world
//! and know nothing about atoms and their inner structure.
//! 
use std::mem::size_of;
use log::{*};
use crate::{inf, sec};
use crate::io::{events::EVENT_SET_DOT, events::EVENT_MOVE_DOT, Param, IO};
use crate::utils;
use crate::global::{Atom, ATOM_TYPE_MASK, Offs, Dir, ATOM_EMPTY, DIRS_LEN, I};
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
    /// All possible directions of nearest atoms
    ///
    pub dirs: [i32; DIRS_LEN],
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
    pub fn new(width: usize, height: usize, dirs: [i32; 8]) -> World {
        sec!("Create world");
        let size = width * height;
        if size < 1 { panic!("Incorrect world size. Size: ({},{})", width, height) }
        inf!("Size: {}x{} atoms, {:?}b", width, height, size * size_of::<Atom>());
        let mut mem = utils::alloc(size);
        utils::zero(&mut mem, 0);

        World {
            width,
            height,
            dirs,
            cells: mem,
            size
        }
    }
    ///
    /// Returns direction between two dots in a world. The direction is
    /// obtained from the perspective of first dot. This function assumes
    /// that two offsets are near each other (distance == 1) and within
    /// the world
    ///
    // pub fn get_dir(offs0: Offs, offs1: Offs) -> Dir {
    //     // 0, 1, 2
    //     if offs1 < offs0 - 1 {}
    //     // 4, 5, 6
    //     else if offs1 > offs0 + 1 {}
    //     else if offs0 - 1 == offs1 { return 7 }
    //     3
    // }
    ///
    /// Returns new offset by start offset and direction. New offset means
    /// a pixel just near specified. Example: F - offset, 0..7 - directions,
    /// horizontal digits - X coordinate, vertical - Y coordinate. Offset F is
    /// equal to (3,2). If direction = 3, than get_offs() returns offset of
    /// (4,2). If direction = 6, than get_offs() returns offset of (2,3)
    /// 
    ///   y
    /// x 01234
    ///   1 012
    ///   2 7F3
    ///   3 654
    ///
    pub fn get_offs(&self, offs: Offs, dir: Dir) -> Offs {
        let offs = offs + self.dirs[dir as I] as Offs;
        if offs < 0 { return self.size as Offs + offs }
        else if offs >= self.size as Offs { return offs - self.size as Offs }
        offs
    }
    ///
    /// Return true if there is an atom under specified offset
    ///
    pub fn is_atom(&self, offs: Offs) -> bool {
        if offs >= self.size as Offs { return false }
        (self.cells[offs as I] & ATOM_TYPE_MASK) != ATOM_EMPTY
    }

    pub fn get_atom(&self, offs: Offs) -> Atom {
        if offs >= self.size as Offs { return ATOM_EMPTY }
        self.cells[offs as I]
    }

    pub fn get_dir_atom(&self, offs: Offs, dir: Dir) -> Atom {
        let offs = offs + self.dirs[dir as I] as Offs;
        if offs < 0 || offs >= self.size as Offs { return ATOM_EMPTY }
        self.cells[offs as I]
    }

    pub fn set_atom(&mut self, offs: Offs, dot: Atom, io: &IO) {
        if offs >= self.size as Offs { return }
        self.cells[offs as I] = dot;
        io.fire(EVENT_SET_DOT, &Param::SetDot(offs, dot));
    }

    pub fn mov_atom(&mut self, src_offs: Offs, dest_offs: Offs, io: &IO) {
        if src_offs >= self.size as Offs || dest_offs >= self.size as Offs { return }
        let atom = self.cells[src_offs as I];
        self.cells[dest_offs as I] = atom;
        self.cells[src_offs as I] = ATOM_EMPTY;
        io.fire(EVENT_MOVE_DOT, &Param::MoveDot(src_offs, dest_offs, atom));
    }
}

#[cfg(test)]
mod tests {
    use crate::global::{ATOM_EMPTY, DIR_UP, DIR_UP_RIGHT, DIR_RIGHT, DIR_RIGHT_DOWN};
    use crate::global::{DIR_DOWN, DIR_DOWN_LEFT, DIR_LEFT, DIR_LEFT_UP, Offs};
    use crate::cfg::Config;
    use crate::io::IO;

    use super::World;
    #[test]
    fn test_new() {
        let w: i32 = 10;
        let world = World::new(w as usize, w as usize, Config::get_dir_offs(w));

        assert_eq!(world.width, w as usize);
        assert_eq!(world.height, w as usize);
        assert_eq!(world.get_atom(0), ATOM_EMPTY);
        assert_eq!(world.get_atom(10), ATOM_EMPTY);
        assert_eq!(world.get_atom(90), ATOM_EMPTY);
    }
    #[test]
    #[should_panic]
    fn test_new_zero_size() {
        let w: i32 = 0;
        World::new(w as usize, w as usize, Config::get_dir_offs(w));
    }
    #[test]
    fn test_get_offs() {
        let w: i32 = 10;
        let world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        assert_eq!(world.get_offs(11, DIR_UP), 1);
        assert_eq!(world.get_offs(11, DIR_UP_RIGHT), 2);
        assert_eq!(world.get_offs(11, DIR_RIGHT), 12);
        assert_eq!(world.get_offs(11, DIR_RIGHT_DOWN), 22);
        assert_eq!(world.get_offs(11, DIR_DOWN), 21);
        assert_eq!(world.get_offs(11, DIR_DOWN_LEFT), 20);
        assert_eq!(world.get_offs(11, DIR_LEFT), 10);
        assert_eq!(world.get_offs(11, DIR_LEFT_UP), 0);
    }
    #[test]
    fn test_get_offs_min_max() {
        let w: i32 = 10;
        let world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        assert_eq!(world.get_offs(0, DIR_UP), (w * w - 1) as Offs);
        assert_eq!(world.get_offs(0, DIR_UP_RIGHT), (w * w - 1) as Offs);
        assert_eq!(world.get_offs(0, DIR_LEFT), (w * w - 1) as Offs);
        assert_eq!(world.get_offs(0, DIR_LEFT_UP), (w * w - 1) as Offs);

        assert_eq!(world.get_offs(99, DIR_RIGHT), 0);
        assert_eq!(world.get_offs(99, DIR_RIGHT_DOWN), 0);
        assert_eq!(world.get_offs(99, DIR_DOWN), 0);
        assert_eq!(world.get_offs(99, DIR_DOWN_LEFT), 0);
    }
    #[test]
    fn test_get_atom() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 65535;
        let io = IO::new();
        assert_eq!(world.get_atom(0), ATOM_EMPTY);
        world.set_atom(0, atom, &io);
        assert_eq!(world.get_atom(0), atom);
        for i in 1..w * w { assert_eq!(world.get_atom(i as Offs), ATOM_EMPTY) }
    }
    #[test]
    fn test_get_atom_min_max() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 65535;
        let io = IO::new();
        assert_eq!(world.get_atom(0), ATOM_EMPTY);
        world.set_atom(0, atom, &io);
        world.set_atom((w * w - 1) as Offs, atom, &io);
        assert_eq!(world.get_atom(0), atom);
        for i in 1..w * w - 1 { assert_eq!(world.get_atom(i as Offs), ATOM_EMPTY) }
    }
    #[test]
    fn test_get_dir_atom() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 65535;
        let io = IO::new();
        assert_eq!(world.get_atom(0), ATOM_EMPTY);
        world.set_atom(11, atom, &io);

        assert_eq!(world.get_dir_atom(1, DIR_DOWN), atom);
        assert_eq!(world.get_dir_atom(2, DIR_DOWN_LEFT), atom);
        assert_eq!(world.get_dir_atom(12, DIR_LEFT), atom);
        assert_eq!(world.get_dir_atom(22, DIR_LEFT_UP), atom);
        assert_eq!(world.get_dir_atom(21, DIR_UP), atom);
        assert_eq!(world.get_dir_atom(20, DIR_UP_RIGHT), atom);
        assert_eq!(world.get_dir_atom(10, DIR_RIGHT), atom);
        assert_eq!(world.get_dir_atom(0, DIR_RIGHT_DOWN), atom);
    }
    #[test]
    fn test_get_dir_atom_min_max() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 65535;
        let io = IO::new();
        assert_eq!(world.get_atom(0), ATOM_EMPTY);
        assert_eq!(world.get_atom((w * w - 1) as Offs), ATOM_EMPTY);
        world.set_atom(0, atom, &io);
        world.set_atom((w * w - 1) as Offs, atom, &io);

        assert_eq!(world.get_dir_atom(0, DIR_UP), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(0, DIR_UP_RIGHT), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(0, DIR_LEFT), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(0, DIR_LEFT_UP), ATOM_EMPTY);

        assert_eq!(world.get_dir_atom(99, DIR_RIGHT), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(99, DIR_RIGHT_DOWN), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(99, DIR_DOWN), ATOM_EMPTY);
        assert_eq!(world.get_dir_atom(99, DIR_DOWN_LEFT), ATOM_EMPTY);
    }
    #[test]
    fn test_set_atom() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 128;
        let io = IO::new();

        for a in 0..(w * w - 1) { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.set_atom(0, atom, &io);
        assert_eq!(world.get_atom(0 as Offs), atom);
        for a in 1..(w * w - 1) { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.set_atom(10, atom, &io);
        assert_eq!(world.get_atom(0 as Offs), atom);
        for a in 1..10 { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        for a in 11..(w * w - 1) { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
    }
    #[test]
    fn test_set_atom_max() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 128;
        let max_offs: Offs = (w * w - 1) as Offs;
        let io = IO::new();

        for a in 0..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.set_atom((w * w * w) as Offs, atom, &io);
        for a in 0..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
    }
    #[test]
    fn test_mov_atom() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 128;
        let max_offs: Offs = (w * w - 1) as Offs;
        let io = IO::new();

        for a in 0..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.set_atom(0, atom, &io);
        world.mov_atom(0, 1, &io);
        assert_eq!(world.get_atom(1), atom);
        for a in 2..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.mov_atom(1, 0, &io);
        for a in 1..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
    }
    #[test]
    fn test_mov_atom_min_max() {
        let w: i32 = 10;
        let mut world = World::new(w as usize, w as usize, Config::get_dir_offs(w));
        let atom = 128;
        let max_offs: Offs = (w * w - 1) as Offs;
        let io = IO::new();

        for a in 0..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
        world.set_atom(0, atom, &io);
        world.mov_atom(0, max_offs + 1, &io);
        assert_eq!(world.get_atom(0), atom);
        for a in 1..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }

        world.mov_atom(max_offs + 1, 1, &io);
        assert_eq!(world.get_atom(0), atom);
        for a in 1..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }

        world.mov_atom(max_offs + 1, max_offs + 2, &io);
        assert_eq!(world.get_atom(0), atom);
        for a in 1..max_offs { assert_eq!(world.get_atom(a as Offs), ATOM_EMPTY) }
    }
}