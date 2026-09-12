use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::fmt;
use std::io;
use std::fs;

use plugin_sdk::plugin::PluginInfo;
use plugin_sdk::query::schema::QuerySchema;

use super::wasm::{self, CallError};

// ── Config ────────────────────────────────────────────────────────────────────

const WASM_EXTENSION: &str = "wasm";

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum RegistryError {
    Io(io::Error),
    Call(CallError),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::Io(e)   => write!(f, "IO error: {e}"),
            RegistryError::Call(e) => write!(f, "Failed to load plugin: {e}"),
        }
    }
}

impl From<io::Error> for RegistryError {
    fn from(e: io::Error) -> Self { RegistryError::Io(e) }
}
impl From<CallError> for RegistryError {
    fn from(e: CallError) -> Self { RegistryError::Call(e) }
}

#[derive(Clone)]
pub struct Plugin {
    pub info:   PluginInfo,
    pub schema: Arc<QuerySchema>,
    pub wasm:   Arc<Vec<u8>>,
}

// ── Registry ──────────────────────────────────────────────────────────────────

pub struct PluginRegistry {
    plugins: HashMap<String, Plugin>,
}

impl PluginRegistry {
    // ── Public ────────────────────────────────────────────────────────────────

    pub fn load_from_dir(dir: &Path) -> Self {
        let mut plugins = HashMap::new();

        let entries = match fs::read_dir(dir) {
            Ok(e)  => e,
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    eprintln!("[plugins] Cannot read plugins directory {:?}: {e}", dir);
                }
                else if let Err(e) = fs::create_dir_all(dir) {
                    eprintln!("[plugins] Could not create plugins directory {:?}: {e}", dir);
                } else {
                    println!("[plugins] Created plugins directory {:?}", dir);
                }

                return PluginRegistry { plugins };
            }
        };

        for entry in entries.flatten() {
            Self::try_load_entry(&entry, &mut plugins);
        }

        PluginRegistry { plugins }
    }

    pub fn get(&self, id: &str) -> Option<&Plugin> {
        self.plugins.get(id)
    }

    pub fn plugins(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins.values()
    }

    // ── Private ───────────────────────────────────────────────────────────────

    fn try_load_entry(entry: &fs::DirEntry, plugins: &mut HashMap<String, Plugin>) {
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some(WASM_EXTENSION) {
            return;
        }

        match Self::load(&path) {
            Ok(plugin) => {
                println!("[plugins] Loaded \"{}\" from {:?}", plugin.info.id, path);
                plugins.insert(plugin.info.id.clone(), plugin);
            }
            Err(e) => eprintln!("[plugins] Failed to load {:?}: {e}", path),
        }
    }

    fn load(path: &Path) -> Result<Plugin, RegistryError> {
        let wasm_bytes = fs::read(path)?;
        let info = wasm::plugin_info(&wasm_bytes)?;
        let schema = wasm::schema(&wasm_bytes)?;

        Ok(Plugin { info, schema: Arc::new(schema), wasm: Arc::new(wasm_bytes) })
    }
}
