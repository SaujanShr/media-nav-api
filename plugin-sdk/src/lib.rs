mod category;

pub use category::Category;

use serde::Serialize;

/// Descriptive information about a plugin, shown in catalogues and UIs.
#[derive(Serialize, Clone, Copy)]
pub struct PluginMetadata {
    pub name:        &'static str,
    pub description: &'static str,
    pub category:    Category,
    pub nsfw:        bool,
}

/// Static assets the plugin ships — loaded by the frontend at runtime.
#[derive(Serialize, Clone, Copy)]
pub struct PluginResources {
    pub icon_url:   &'static str,
    pub banner_url: &'static str,
}

/// The universal concrete plugin type.
///
/// Every plugin's `create_plugin` entry-point returns a heap-allocated `Plugin`.
/// The server deserializes it directly — no custom struct or trait impl needed.
///
/// ```rust
/// #[unsafe(no_mangle)]
/// pub extern "C" fn create_plugin() -> *mut Plugin {
///     Box::into_raw(Box::new(Plugin {
///         id:        "my-plugin",
///         version:   "1.0.0",
///         metadata:  PluginMetadata { ... },
///         resources: PluginResources { ... },
///     }))
/// }
/// ```
#[derive(Serialize, Clone, Copy)]
pub struct Plugin {
    pub id:        &'static str,
    pub version:   &'static str,
    pub metadata:  PluginMetadata,
    pub resources: PluginResources,
}
