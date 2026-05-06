use serde::Serialize;

use crate::category::Category;
use crate::library_item::{LibraryItem, LibraryItemDetail};

/// Parameters passed to a plugin's fetch function.
pub struct FetchRequest {
    pub page:      u32,
    pub page_size: u32,
}

/// The paginated result returned by a plugin's fetch function.
#[derive(Serialize)]
pub struct FetchResult {
    pub items: Vec<LibraryItem>,
    pub total: u64,
}

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
/// The server loads it directly — no custom struct or trait impl needed.
///
/// ```rust
/// #[unsafe(no_mangle)]
/// pub extern "C" fn create_plugin() -> *mut Plugin {
///     Box::into_raw(Box::new(Plugin {
///         id:        "my-plugin",
///         version:   "1.0.0",
///         metadata:  PluginMetadata { ... },
///         resources: PluginResources { ... },
///         fetch:     my_fetch_fn,
///     }))
/// }
/// ```
#[derive(Serialize, Clone, Copy)]
pub struct Plugin {
    pub id:        &'static str,
    pub version:   &'static str,
    pub metadata:  PluginMetadata,
    pub resources: PluginResources,
    #[serde(skip)]
    pub fetch:  fn(FetchRequest) -> FetchResult,
    #[serde(skip)]
    pub enrich: fn(&str) -> Option<LibraryItemDetail>,
}

