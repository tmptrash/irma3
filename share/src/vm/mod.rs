//!
//! Virtual Machine module. Implements all atom types and related stuff. Should be
//! optimized for speed. There are many virtual machines in a world at the same time.
//! One VM runs one molecule.
//! 
pub mod buf;
pub mod vmdata;

use crate::Core;
use crate::global::Atom;
use crate::global::{*};
use crate::atom::{*};
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOMS_MAP: &[fn(&mut VM, Atom, &mut Core) -> bool] = &[
    VM::atom_empty,  // 0 - must be an empty fn. Means empty cell or no atom
    VM::atom_mov,
    VM::atom_fix,
    VM::atom_spl,
    VM::atom_if,
    VM::atom_job,
    VM::atom_empty,  // unused
    VM::atom_empty   // unused
];
///
/// Index of if atom. Must be synchronized with ATOMS_MAP
///
const ATOM_IF: Atom = 4;
///
/// Describes data for one instance of Virtual Machine
///
#[derive(Copy, Clone)]
pub struct VM {
    ///
    /// Energy of current VM. Every VM may have it's own.
    ///
    energy: isize,
    ///
    /// Offset of current atom, which VM in running.
    ///
    offs: Offs
}

impl VM {
    pub fn new(energy: isize, offs: Offs) -> VM {
        VM {
            energy,
            offs
        }
    }
    ///
    /// Runs one atom depending on type and moves VM to the next one depending on
    /// atom direction.
    ///
    pub fn run_atom(&mut self, core: &mut Core) -> bool {
        let atom: Atom = core.vm_data.world.get_atom(self.offs);
        let atom_type = get_type(atom);
        if atom_type == ATOM_EMPTY { return false }

        ATOMS_MAP[atom_type as I](self, atom, core)
    }
    ///
    /// Returns energy amount of current VM
    ///
    pub fn get_energy(&self) -> isize { self.energy }
    ///
    /// Returns offset of current VM
    ///
    pub fn get_offs(&self) -> Offs { self.offs }
    ///
    /// Implements mov command. It moves current atom and all binded atoms together.
    /// Should be optimized by speed. After moving all bonds should not be broken.
    ///
    fn atom_mov(&mut self, mut atom: Atom, core: &mut Core) -> bool {
        let mut offs: Offs;
        let mut to_offs: Offs;
        let mut dir0: Dir;
        let mut dir1: Dir;
        let mut a: Atom;
        let mut o: Offs;
        let dir = get_dir1(atom);                                         // atom move direction
        let stack = &mut core.vm_data.buf.stack;
        let wrld = &mut core.vm_data.world;
        let moved = &mut core.vm_data.buf.buf;
        let mov_energy = core.cfg.atoms().mov_energy;
        let vm_offs = wrld.get_offs(self.offs, dir);

        stack.clear();                                                    // every call of mov should reset stack & buf
        moved.clear();
        stack.push(self.offs);

        while !stack.empty() {                                            // before while, stack should have >= 1 atom
            offs = stack.last().unwrap();                                 // offset of atom before move
            if moved.contains(&offs) || !wrld.is_atom(offs) {             // this atom was already moved or it doesn't exist
                stack.shrink();
                continue;
            }
            to_offs = wrld.get_offs(offs, dir);                           // destination atom position
            if wrld.is_atom(to_offs) {                                    // can't move atom. Another one is there
                stack.push(to_offs);
                continue;
            }
            stack.shrink();                                               // destination cell is empty, can move there
            wrld.mov_atom(offs, to_offs, &core.io);                       // move atom physically
            moved.insert(to_offs);                                        // mark atom as "already moved"
            self.energy -= mov_energy;                                    // decrease energy for every moved atom
            atom = wrld.get_atom(to_offs);                                // atom we have to move
            // update vm bond of moved atom---------------------------------------------------------------------------------
            dir0 = get_vm_dir(atom);                                      // get VM dir of moved atom
            if dir0 != DIR_NO {                                           // there is no near atom
                dir1 = DIR_MOV_ATOM[dir0 as I][dir as I];                 // final dir of moved atom
                o = wrld.get_offs(offs, dir0);                            // offs of near atom
                if dir1 == DIR_NO {                                       // near atom is to far, will add it later
                    if wrld.is_atom(o) && dir != dir0 { stack.push(o); }
                } else {
                    set_vm_dir(&mut atom, dir1);                          // distance between atoms is 1. update bond
                    wrld.set_atom(to_offs, atom, &core.io);
                    // update vm bond of near atom------------------------------------------------------------------------------
                    dir0 = DIR_REV[dir0 as I];                            // get near atom's dir to moved atom
                    a = wrld.get_atom(o);                                 // near atom
                    if get_vm_dir(a) == dir0 {                            // near atom has vm bond with moved
                        dir1 = DIR_NEAR_ATOM[dir0 as I][dir as I];        // final dir of near atom
                        set_vm_dir(&mut a, dir1);
                        wrld.set_atom(o, a, &core.io);
                    }
                    // update then bond of near atom---------------------------------------------------------------------------
                    if get_type(a) == ATOM_IF && get_dir2(a) == dir0 {    // near atom has a then bond with moved
                        dir1 = DIR_NEAR_ATOM[dir0 as I][dir as I];        // final dir of near atom
                        set_dir2(&mut a, dir1);
                        wrld.set_atom(o, a, &core.io);
                    }
                }
            }
            if get_type(atom) == ATOM_IF {                                // if atom has additional then bond
                // update then bond of moved atom---------------------------------------------------------------------------
                dir0 = get_dir2(atom);                                      // get then dir of if moved atom
                if dir0 != DIR_NO {                                       // there is no near atom
                    dir1 = DIR_MOV_ATOM[dir0 as I][dir as I];                 // final dir of if moved atom
                    o  = wrld.get_offs(offs, dir0);                     // offs of near atom
                    if dir1 == DIR_NO {                                   // near atom is to far, will add it later
                        if wrld.is_atom(o) && dir != dir0 { stack.push(o); }
                    } else {
                        set_dir2(&mut atom, dir1);                      // distance between atoms is 1. update bond
                        wrld.set_atom(to_offs, atom, &core.io);
                        // update then bond of near atom------------------------------------------------------------------------
                        dir0 = DIR_REV[dir0 as I];                        // get near atom's dir to moved atom
                        a  = wrld.get_atom(o);                            // near atom
                        if get_dir2(a) == dir0 {                          // near atom has a bond with moved
                            dir1 = DIR_NEAR_ATOM[dir0 as I][dir as I];    // final dir of near atom
                            set_dir2(&mut a, dir1);
                            wrld.set_atom(o, a, &core.io);
                        }
                        // update vm bond of near atom--------------------------------------------------------------------------
                        if get_vm_dir(a) == dir0 {                        // near atom has vm bond with moved
                            dir1 = DIR_NEAR_ATOM[dir0 as I][dir as I];    // final dir of near atom
                            set_vm_dir(&mut a, dir1);
                            wrld.set_atom(o, a, &core.io);
                        }
                    }
                }
            }
        }
        a = wrld.get_atom(vm_offs);
        if a != ATOM_EMPTY && has_vm_bond(a) {                            // update VM offs of current atom
            self.offs = wrld.get_offs(vm_offs, get_vm_dir(a));
        }

        true
    }
    ///
    /// Implements fix atom. Creates vm bond between two atoms. If vm bond is already exist, than
    /// try to create if/then bond for if atom. Consumes energy.
    ///
    fn atom_fix(&mut self, atom: Atom, core: &mut Core) -> bool {
        let offs0 = core.vm_data.world.get_offs(self.offs, get_dir1(atom)); // gets first near atom offs to fix
        let mut atom0 = core.vm_data.world.get_atom(offs0);               // gets first near atom to fix
        if !is_atom(atom0) { return false }                               // no first near atom to fix
        let d0 = get_dir2(atom);
        if !is_atom(core.vm_data.world.get_dir_atom(offs0, d0)) { return false } // there is no second near atom to fix

        // fix vm bond------------------------------------------------------------------------------------------------------
        if !has_vm_bond(atom0) {                                          // first near atom has no vm bond
            set_vm_dir(&mut atom0, d0);
            core.vm_data.world.set_atom(offs0, atom0, &core.io);
            if has_vm_bond(atom) { self.offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy -= core.cfg.atoms().fix_energy;
            return true;
        }
        if get_type(atom0) != ATOM_IF { return false }                    // only if atom has if and then bonds
        // fix then bond----------------------------------------------------------------------------------------------------
        if !has_dir2_bond(atom0) {                                       // first near atom has no then bond
            set_dir2(&mut atom0, d0);
            set_dir2_bond(&mut atom0);
            core.vm_data.world.set_atom(offs0, atom0, &core.io);
            if has_vm_bond(atom) { self.offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy -= core.cfg.atoms().fix_energy;
            return true;
        }

        false
    }
    ///
    /// Implements spl atom. Splits two atoms bonds. If atoms has no vm bond, than
    /// try to split if/then bonds for if atom. Releases energy.
    ///
    fn atom_spl(&mut self, atom: Atom, core: &mut Core) -> bool {
        let offs0 = core.vm_data.world.get_offs(self.offs, get_dir1(atom)); // gets first near atom offs to split
        let mut atom0 = core.vm_data.world.get_atom(offs0);               // gets first near atom to split
        if !is_atom(atom0) { return false }                               // no first near atom to split
        let d0 = get_dir2(atom);
        if !is_atom(core.vm_data.world.get_dir_atom(offs0, d0)) { return false }  // there is no second near atom to split

        // split vm bond----------------------------------------------------------------------------------------------------
        if has_vm_bond(atom0) {                                           // first near atom has vm bond
            reset_vm_bond(&mut atom0);
            core.vm_data.world.set_atom(offs0, atom0, &core.io);
            if has_vm_bond(atom) { self.offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy += core.cfg.atoms().spl_energy;
            return true;
        }
        if get_type(atom0) != ATOM_IF { return false }
        // split then bond--------------------------------------------------------------------------------------------------
        if has_dir2_bond(atom0) {                                         // first near atom has then bond
            reset_dir2_bond(&mut atom0);
            core.vm_data.world.set_atom(offs0, atom0, &core.io);
            if has_vm_bond(atom) { self.offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom)) }
            self.energy += core.cfg.atoms().spl_energy;
            return true;
        }

        false
    }
    ///
    /// Implements cond command. Depending on the condition VM will run one of two
    /// possible atoms.
    ///
    fn atom_if(&mut self, atom: Atom, core: &mut Core) -> bool {
        // runs if -> then scenario
        if has_dir2_bond(atom) && is_atom(core.vm_data.world.get_dir_atom(self.offs, get_dir1(atom))) {
            self.offs = core.vm_data.world.get_offs(self.offs, get_dir2(atom));
            self.energy -= core.cfg.atoms().if_energy;
            return true;
        }
        // runs else scenario
        if has_vm_bond(atom) {
            self.offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom));
            self.energy -= core.cfg.atoms().if_energy;
            return true;
        }

        false
    }
    ///
    /// Implements job command. Creates one new VM instance (thread). Energy decreasing
    /// should be called from outside, because new VM is added there
    ///
    fn atom_job(&mut self, atom: Atom, core: &mut Core) -> bool {
        let offs = core.vm_data.world.get_offs(self.offs, get_vm_dir(atom));
        if !is_atom(core.vm_data.world.get_atom(offs)) { return false }
        let energy = self.energy / 2;
        self.energy -= energy;
        if !core.vms.full() {
            core.vms.add(VM::new(energy, offs));
            return true;
        }

        false
    }
    ///
    /// Just a stub for empty atom in a world
    ///
    fn atom_empty(&mut self, _atom: Atom, _core: &mut Core) -> bool { false }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::{fs, path::Path};
    use crate::global::{ATOM_EMPTY, Offs, Atom};
    use crate::world::World;
    use crate::{cfg::Config, Core, io::IO, vm::vmdata::VMData};
    use crate::utils::{id, vec::Vector};

    use super::VM;

    fn create_file(file: &str, content: &str) {
        assert_eq!(fs::write(file, content).is_ok(), true);
    }
    fn remove_file(file: &str) {
        if Path::new(file).exists() { assert_eq!(fs::remove_file(file).is_ok(), true) }
    }
    fn init(vms: i32) -> (String, *mut c_void) {
        let cfg_file = id() + ".json";
        create_file(&cfg_file, &format!(r#"{{"WIDTH": 10, "HEIGHT": 10, "MAX_VM_AMOUNT": {}}}"#, vms));

        let cfg = Config::new(&cfg_file);
        let vm_amount = cfg.MAX_VM_AMOUNT();
        let width = cfg.WIDTH();
        let height = cfg.HEIGHT();
        let dir2offs = cfg.DIR_TO_OFFS();
        let mov_buf_size = cfg.MOV_BUF_SIZE();
        let core = Box::into_raw(Box::new(Core {
            cfg,
            vms: Vector::new(vm_amount),
            io: IO::new(),
            vm_data: VMData::new(width, height, dir2offs, mov_buf_size)
        })).cast();

        (cfg_file, core)
    }
    fn check_offs(offs: &Vec<(Offs, Atom)>, world: &World, size: usize) {
        for o in 0..size {
            let a = world.get_atom(o as isize);
            if !offs.contains(&(o as isize, a)) { assert_eq!(a, ATOM_EMPTY) }
        }
        for (o, a) in offs { assert_eq!(world.get_atom(*o), *a) }
    }
    fn set_atoms(atoms: &Vec<(Offs, Atom)>, world: &mut World, io: &IO, size: usize) {
        for (o, a) in atoms {
            world.set_atom(*o, *a, io);
        }
        check_offs(&atoms, world, size);
    }

    #[test]
    fn test_run_atom_empty() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom = 0b0010_0000_1100_0000;

        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(1, atom)], &mut pvmdata.world, pio, 100); // atom: mov right
        pvms.data[0].run_atom(pcore);
        check_offs(&vec![(1, atom)], &pvmdata.world, 100);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_run_atom_spl() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom = 0b0110_0000_1100_0000;

        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(0, atom)], &mut pvmdata.world, pio, 100); // atom spl
        pvms.data[0].run_atom(pcore);
        check_offs(&vec![(0, atom)], &pvmdata.world, 100);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_one_atom_mov() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom = 0b0010_0000_1100_0000;

        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(0, atom)], &mut pvmdata.world, pio, 100); // atom mov right
        pvms.data[0].atom_mov(atom, pcore);
        assert_eq!(pvmdata.world.get_atom(1), atom);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov0() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0010_1110_1100_0000; // mov
        let atom1 = 0b0110_0000_1100_0000; // spl

        // atoms: [m]->[s]
        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(0, atom0), (1, atom1)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom0, pcore);
        check_offs(&vec![(1, atom0), (2, atom1)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 2);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov1() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0010_1110_1100_0000; // mov
        let atom1 = 0b0111_1110_0000_0000; // spl

        // atoms: [m]=[s]
        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(0, atom0), (1, atom1)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom0, pcore);
        check_offs(&vec![(1, atom0), (2, atom1)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 2);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov2() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0011_0010_1100_0000; // mov
        let atom1 = 0b0110_0010_0000_0000; // spl

        // atoms: [m]
        //          \\
        //           [s]
        pvms.add(VM::new(100, 0));
        set_atoms(&vec![(0, atom0), (11, atom1)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom0, pcore);
        check_offs(&vec![(1, 0b0011_0110_1100_0000), (11, 0b0110_0110_0000_0000)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 11);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov3() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0110_1110_0000_0000; // spl
        let atom1 = 0b0011_1110_1100_0000; // mov

        // atoms: [s]=[m] ->
        pvms.add(VM::new(100, 1));
        set_atoms(&vec![(0, atom0), (1, atom1)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom1, pcore);
        check_offs(&vec![(1, atom0), (2, atom1)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 1);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov4() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0111_0010_0000_0000; // spl
        let atom1 = 0b0010_0011_0000_0000; // mov

        // atoms: [s]
        //          \\
        //           [m]
        //              v
        pvms.add(VM::new(100, 11));
        set_atoms(&vec![(0, atom0), (11, atom1)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom1, pcore);
        check_offs(&vec![(11, atom0), (22, atom1)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 11);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov5() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0110_1110_0000_0000; // spl
        let atom1 = 0b0011_1110_1100_0000; // mov
        let atom2 = 0b0110_0000_0000_0000; // spl

        // atoms: [s]=[m][s] >
        pvms.add(VM::new(100, 1));
        set_atoms(&vec![(0, atom0), (1, atom1), (2, atom2)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom1, pcore);
        check_offs(&vec![(1, atom0), (2, atom1), (3, atom2)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 1);

        remove_file(&cfg_file);
    }
    #[test]
    fn test_two_atom_mov6() {
        let (cfg_file, core) = init(1);
        let pvms = unsafe{ &mut (*(core as *mut Core)).vms };
        let pvmdata = unsafe{ &mut (*(core as *mut Core)).vm_data };
        let pio = unsafe{ &mut (*(core as *mut Core)).io };
        let pcore = unsafe{ &mut *(core as *mut Core) };
        let atom0 = 0b0110_0000_0000_0000; // spl
        let atom1 = 0b0110_0000_0000_0000; // spl
        let atom2 = 0b0010_0010_1100_0000; // mov

        // atoms: [s][s]
        //          \\
        //           [m] >
        pvms.add(VM::new(100, 11));
        set_atoms(&vec![(0, atom0), (1, atom1), (11, atom2)], &mut pvmdata.world, pio, 100);
        pvms.data[0].atom_mov(atom2, pcore);
        check_offs(&vec![(1, atom0), (2, atom1), (12, atom2)], &pvmdata.world, 100);
        assert_eq!(pvms.data[0].get_offs(), 1);

        remove_file(&cfg_file);
    }
}