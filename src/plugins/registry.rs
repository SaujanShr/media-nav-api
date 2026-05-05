use std::collections::HashMap;
use std::path::Path;
use std::fmt;
use std::io;
use std::fs;
use std::ffi;

use libloading::{Library, Symbol};

use super::PluginInstance;

type CreatePluginFn = unsafe extern "C" fn() -> *mut ffi::c_void;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum RegistryError {
    Io(io::Error),
    Load(libloading::Error),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::Io(e)   => write!(f, "IO error: {e}"),
            RegistryError::Load(e) => write!(f, "Failed to load plugin: {e}"),
        }
    }
}

impl From<io::Error> for RegistryError {
    fn from(e: io::Error) -> Self {
        RegistryError::Io(e)
    }
}
impl From<libloading::Error> for RegistryError {
    fn from(e: libloading::Error) -> Self {
        RegistryError::Load(e)
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────

pub struct PluginRegistry {
    _libraries: Vec<Library>,
    plugins: HashMap<String, Box<dyn PluginInstance>>,
}

impl PluginRegistry {
    pub fn load_from_dir(dir: &Path) -> Self {
        let mut libraries = Vec::new();
        let mut plugins: HashMap<String, Box<dyn PluginInstance>> = HashMap::new();

        let ext = if cfg!(target_os = "macos") { "dylib" } else { "so" };

        let entries = match fs::read_dir(dir) {
            Ok(e)  => e,
            Err(e) => {
                eprintln!("[plugins] Cannot read plugins directory {:?}: {e}", dir);
                return PluginRegistry { _libraries: libraries, plugins };
            }
        };

        for entry in entries.flatten() {
            Self::try_load_entry(&entry, ext, &mut plugins, &mut libraries);
        }

        PluginRegistry { _libraries: libraries, plugins }
    }

    pub fn get(&self, id: &str) -> Option<&dyn PluginInstance> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    pub fn is_loaded(&self, id: &str) -> bool {
        self.plugins.contains_key(id)
    }

    pub fn loaded_ids(&self) -> impl Iterator<Item = &str> {
        self.plugins.keys().map(String::as_str)
    }

    fn try_load_entry(
        entry: &fs::DirEntry,
        ext: &str,
        plugins: &mut HashMap<String, Box<dyn PluginInstance>>,
        libraries: &mut Vec<Library>,
    ) {
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some(ext) {
            return;
        }

        match Self::load(&path) {
            Ok((lib, plugin)) => {
                println!("[plugins] Loaded \"{}\" from {:?}", plugin.id(), path);
                plugins.insert(plugin.id().to_string(), plugin);
                libraries.push(lib);
            }
            Err(e) => eprintln!("[plugins] Failed to load {:?}: {e}", path),
        }
    }

    fn load(path: &Path) -> Result<(Library, Box<dyn PluginInstance>), RegistryError> {
        unsafe {
            let lib = Library::new(path)?;
            let create: Symbol<CreatePluginFn> = lib.get(b"create_plugin")?;
            let plugin = *Box::from_raw(create() as *mut Box<dyn PluginInstance>);
            Ok((lib, plugin))
        }
    }
}

