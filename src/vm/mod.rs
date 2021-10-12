pub mod buf;

use std::rc::Rc;
use std::cell::RefCell;
use crate::world::World;
use crate::global::Atom;
use buf::MoveBuffer;

pub struct VM {
    //
    // Energy of current VM. Every VM may have it's own
    //
    energy: usize,
    //
    // Offset of current atom, which VM in running
    //
    offs: usize,
    //
    // Shared between VMs buffer. Is used in mov atom
    //
    buf: Rc<RefCell<MoveBuffer>>
}
//
// map between atom type number and handler fn index. Should be in stack
//
const ATOM_MAP: &'static [fn(&mut VM, &World)] = &[VM::atom_mov, VM::atom_fix, VM::atom_cond, VM::atom_job];
//
// Masks for atom inner bits
//
const ATOM_TYPE_MASK: u16 = 0b11000000_00000000; // We use 0..1 bits for atom type
const ATOM_DIR_MASK:  u16 = 0b00111000_00000000; // We use 2..4 bits for VM run direction
const ATOM_FIX_MASK:  u16 = 0b00000100_00000000; // We use bit 5 as fix/unfix atom type switch
const ATOM_IF_MASK:   u16 = 0b00111000_00000000; // We use 2..4 bits for if direction (if atom)
const ATOM_THEN_MASK: u16 = 0b00000111_00000000; // We use 5..7 bits for then direction (if atom)
const ATOM_CON_MASK:  u16 = 0b00000000_11111111; // We use 8..15 bits for atom connections

impl VM {
    pub fn new(buf: Rc<RefCell<MoveBuffer>>) -> VM {
        VM {
            energy: 0,
            buf: buf,
            offs: 0
        }
    }

    pub fn create_vms(vm_amount: usize, buf_size: usize) -> Vec<VM> {
        let buf = Rc::new(RefCell::new(MoveBuffer::new(buf_size)));
        let mut v: Vec<VM> = Vec::with_capacity(vm_amount); 
        for _i in 0..vm_amount {
            v.push(VM::new(buf.clone()));
        }
        v
    }
    //
    // Run one atom
    //
    pub fn step(&mut self, mut world: &World) {
        let atom:Atom = world.get_dot(self.offs);
        let atom_type: usize = (atom & ATOM_TYPE_MASK).into();
        if (atom_type >= ATOM_MAP.len()) { return }
        ATOM_MAP[atom_type](self, world);
    }
    //
    // Atom handlers
    //
    pub fn atom_mov(&mut self, mut world: &World) {

    }

    pub fn atom_fix(&mut self, mut world: &World) {

    }

    pub fn atom_cond(&mut self, mut world: &World) {

    }

    pub fn atom_job(&mut self, mut world: &World) {

    }
}