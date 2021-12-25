//!
//! Simple Plugin Manager module. Loads all available plugins, finds their API
//! and run them.
//!
use dlopen::wrapper::{Container, WrapperApi};
use log::{*};
use std::fs;
use std::env;
use share::io::IO;
///
/// Plugin API. All required functions of the plugin
///
#[derive(WrapperApi)]
pub struct Plugin {
    init: fn(io: &IO),
    idle: fn(io: &IO),
    remove: fn(io: &IO)
}
///
/// Loads all plugins in configured folder. Returns a vector of dynamic libraries
/// with an API (function pointers).
///
pub fn load(path: &str) -> Vec<Container<Plugin>> {
    info!("  Load core plugins");
    load_libs(path)
}
///
/// Inits plugins. This is a place where plugins may add their listeners to the 
/// Core IO object
///
pub fn init(plugins: &[Container<Plugin>], io: &IO) {
    info!("  Init core plugins");
    for p in plugins.iter() { p.init(io) }
}
///
/// Calls plugins idle() function to do their internal work. On every iteration
/// Core calls this function for every plugin.
///
pub fn idle(plugins: &[Container<Plugin>], io: &IO) { for p in plugins.iter() { p.idle(io) } }
///
/// Destroy all plugins. Destroy means removing of Container<Plugin>
/// structure for plugins.
///
pub fn remove(plugins: &[Container<Plugin>], io: &IO) {
    info!("  Destroy core plugins");
    for p in plugins { p.remove(io) }
}
///
/// Finds all plugins in some folder and returns a vector of them
///
fn load_libs(folder: &str) -> Vec<Container<Plugin>> {
    let mut plugins = Vec::<Container<Plugin>>::new();

    info!("    Looking plugins in \"{}\\{}\"", env::current_dir().unwrap().as_path().display(), folder);
    let files = fs::read_dir(folder);
    if let Result::Err(_e) = files {
        warn!("      Folder \"{}\" is incorrect or doesn't exist", folder);
        return plugins;
    }

    for f in files.unwrap() {
        let plugin = f.unwrap();
        if plugin.path().is_dir() { continue }

        info!("    Found plugin: \"{}\"", plugin.path().display());
        let lib = unsafe { Container::load(plugin.path()) };
        if lib.is_err() {
            warn!("      Invalid plugin: \"{}\"", plugin.path().display());
            continue;
        }
        plugins.push(lib.unwrap());
    }

    plugins
}