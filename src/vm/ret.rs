//!
//! Return value enum. Is used in VM for running atoms and returning
//! their statuses
//!
use crate::global::Offs;

pub enum Return {
    Code(u8),
    AddVm(isize, Offs)
}