//!
//! Simple Plugin Manager module. Loads all available plugins, finds their API
//! and run them.
//!
use dlopen::wrapper::{Container, WrapperApi};
use colored::Colorize;
use log::{*};
use std::fs;
use std::env;
use share::Core;
///
/// Plugin API. All required functions of the plugin
///
#[derive(WrapperApi)]
pub struct Plugin {
    init: fn(core: &mut Core),
    idle: fn(core: &Core),
    remove: fn(core: &Core)
}
///
/// Container of all loaded plugins
///
pub struct Plugins {
    plugins: Vec<Container<Plugin>>,
    names: Vec<String>
}

impl Plugins {
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
            panic!("{}", "Please specify correct plugins folder. See Config.PLUGINS_DIR for details".red().bold());
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

        if self.plugins.is_empty() {
            panic!("{}", "No core plugins were found. Please specify correct plugins folder. See Config.PLUGINS_DIR for details".red().bold());
        }
    }
    ///
    /// Inits plugins. This is a place where plugins may add their listeners to the 
    /// Core IO object
    ///
    pub fn init(&mut self, core: &mut Core) {
        sec!("Init core plugins");
        for (i , p) in self.plugins.iter().enumerate() {
            inf!("Init plugin \"{}\"", self.names.get(i).unwrap());
            p.init(core);
        }
    }
    ///
    /// Calls plugins idle() function to do their internal work. On every iteration
    /// Core calls this function for every plugin.
    ///
    pub fn idle(&self, core: &Core) { for p in self.plugins.iter() { p.idle(core) } }
    ///
    /// Destroy all plugins. Destroy means removing of Container<Plugin>
    /// structure for plugins.
    ///
    pub fn remove(&self, core: &Core) {
        sec!("Destroy core plugins");
        for (i, p) in self.plugins.iter().enumerate() {
            inf!("Remove plugin \"{}\"", self.names.get(i).unwrap());
            p.remove(core);
        }
    }
}

mod tests {
    #[test]
    #[should_panic]
    fn test_load() {
        let folder = "test-666";
        std::fs::create_dir(folder).unwrap();
        let mut plugins = crate::Plugins::new();
        plugins.load(folder);
        assert!(std::fs::remove_dir_all(folder).is_ok());
    }
}