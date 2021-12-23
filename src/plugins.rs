//!
//! Simple Plugin Manager module. Loads all available plugins and run them
//!
use dlopen;
use dlopen::wrapper::{Container, WrapperApi};
use log::{*};
use std::fs;
use std::env;
use std::io;
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
/// Finds all plugins in some folder and returns a vector of them
///
pub fn load(folder: &str) -> Result<Vec<Container<Plugin>>, dlopen::Error> {
    let mut plugins = Vec::<Container<Plugin>>::new();

    info!("    Looking plugins in \"{}\\{}\"", env::current_dir().unwrap().as_path().display(), folder);
    let files = fs::read_dir(folder);
    if let Result::Err(_e) = files {
        warn!("      Folder \"{}\" is incorrect or doesn't exist", folder);
        return Ok(plugins);
    }

    for f in files.unwrap() {
        let plugin = f.unwrap();
        if plugin.path().is_dir() { continue }

        info!("      Found plugin: \"{}\"", plugin.path().display());
        let lib = unsafe { Container::load(plugin.path()) };
        if lib.is_err() {
            warn!("      Invalid plugin: \"{}\"", plugin.path().display());
            continue;
        }
        plugins.push(lib.unwrap());
    }

    Ok(plugins)
}