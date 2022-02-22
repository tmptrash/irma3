pub mod io;
pub mod utils;
pub mod vm;
pub mod atom;
pub mod cfg;
pub mod dump;
pub mod global;
#[macro_use] pub mod logger;
pub mod world;
///
/// Struct for all shared between core and plugins objects (VMs, IO, Config).
/// Using this struct plugins and core may communicate between each other
///
pub struct Core {
    pub cfg: cfg::Config,
    pub vms: utils::vec::Vector<vm::VM>,
    pub io : io::IO,
    pub vm_data: vm::vmdata::VMData
}