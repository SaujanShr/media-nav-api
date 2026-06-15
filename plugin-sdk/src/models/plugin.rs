use serde::Serialize;

use crate::library_item::{LibraryItem, LibraryItemDetail};
use crate::query::{Query, schema::QuerySchema};

pub struct FetchRequest {
    pub page:      u32,
    pub page_size: u32,
    pub query:     Query,
}

#[derive(Serialize)]
pub struct FetchResult {
    pub items: Vec<LibraryItem>,
    pub total: u64,
}

#[derive(Serialize)]
pub struct PluginMetadata {
    pub name:        &'static str,
    pub description: &'static str,
    pub nsfw:        bool,
}

#[derive(Serialize)]
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
///         enrich:    my_enrich_fn,
///     }))
/// }
/// ```
#[derive(Serialize)]
pub struct Plugin {
    pub id:        &'static str,
    pub version:   &'static str,
    pub metadata:  PluginMetadata,
    pub resources: PluginResources,
    #[serde(skip)]
    pub schema:    fn() -> QuerySchema,
    #[serde(skip)]
    pub fetch:     fn(FetchRequest) -> FetchResult,
    #[serde(skip)]
    pub enrich:    fn(&str) -> Option<LibraryItemDetail>,
}
