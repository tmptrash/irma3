//!
//! Simple Plugin Manager module. Loads all available plugins, finds their API
//! and run them.
//!
use dlopen::wrapper::{Container, WrapperApi};
use log::{*};
use std::fs;
use std::env;
use share::io::IO;
use share::cfg::Config;
///
/// Plugin API. All required functions of the plugin
///
#[derive(WrapperApi)]
pub struct Plugin {
    init: fn(io: &mut IO, cfg: &mut Config),
    idle: fn(io: &IO),
    remove: fn(io: &IO)
}
///
/// Container of all loaded plugins
///
pub struct Plugins {
    plugins: Vec<Container<Plugin>>,
    names: Vec<String>
}
impl<'a> Plugins {
    pub fn new() -> Plugins {
        Plugins {
            plugins: Vec::<Container<Plugin>>::new(),
            names: Vec::<String>::new()
        }
    }
    ///
    /// Loads all plugins in configured folder. Returns a vector of dynamic libraries
    /// with an API (function pointers).
    ///
    pub fn load(&mut self, folder: &str) {
        sec!("Load core plugins");
        inf!("Looking plugins in \"{}\\{}\"", env::current_dir().unwrap().as_path().display(), folder);
        let files = fs::read_dir(folder);
        if let Result::Err(_e) = files {
            err!("Folder \"{}\" is incorrect or doesn't exist", folder);
            return;
        }

        for f in files.unwrap() {
            let plugin = f.unwrap();
            if plugin.path().is_dir() { continue }

            inf!("Found plugin: \"{}\"", plugin.path().display());
            match u! { Container::load(plugin.path()) } {
                Err(_) => {
                    err!("Invalid plugin: \"{}\"", plugin.path().display());
                    continue;
                },
                Ok(p) => {
                    self.names.push(plugin.path().display().to_string());
                    self.plugins.push(p);
                }
            }
        }
    }
    ///
    /// Inits plugins. This is a place where plugins may add their listeners to the 
    /// Core IO object
    ///
    pub fn init(&mut self, io: &mut IO, cfg: &mut Config) {
        sec!("Init core plugins");
        for (i , p)in self.plugins.iter().enumerate() {
            inf!("Init plugin \"{}\"", self.names.get(i).unwrap());
            p.init(io, cfg);
        }
    }
    ///
    /// Calls plugins idle() function to do their internal work. On every iteration
    /// Core calls this function for every plugin.
    ///
    pub fn idle(&self, io: &IO) { for p in self.plugins.iter() { p.idle(io) } }
    ///
    /// Destroy all plugins. Destroy means removing of Container<Plugin>
    /// structure for plugins.
    ///
    pub fn remove(&self, io: &IO) {
        sec!("Destroy core plugins");
        for (i, p) in self.plugins.iter().enumerate() {
            inf!("Remove plugin \"{}\"", self.names.get(i).unwrap());
            p.remove(io);
        }
    }
}