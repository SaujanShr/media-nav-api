use plugin_sdk::category::Category;
use plugin_sdk::plugin::{FetchRequest, FetchResult, Plugin, PluginMetadata, PluginResources};
use plugin_sdk::library_item::{LibraryItem, LibraryItemDetail, LibraryItemMetadata, LibraryItemResources};

// ── Plugin ────────────────────────────────────────────────────────────────────

const PLUGIN: Plugin = Plugin {
    id:      "example",
    version: "0.1.0",
    metadata: PluginMetadata {
        name:        "Example Plugin",
        description: "A minimal example plugin for media-nav.",
        category:    Category::Unknown,
        nsfw:        false,
    },
    resources: PluginResources {
        icon_url:   "https://example.com/icon.png",
        banner_url: "https://example.com/banner.png",
    },
    fetch:  mock_fetch,
    enrich: mock_enrich,
};

fn mock_fetch(_req: FetchRequest) -> FetchResult {
    FetchResult {
        items: vec![
            LibraryItem {
                id:            "example-item-1".to_string(),
                title:         "Example Item".to_string(),
                thumbnail_url: "https://example.com/thumb.png".to_string(),
            },
        ],
        total: 1,
    }
}

fn mock_enrich(id: &str) -> Option<LibraryItemDetail> {
    match id {
        "example-item-1" => Some(LibraryItemDetail {
            id: id.to_string(),
            metadata: LibraryItemMetadata {
                title: "Example Item".to_string(),
            },
            resources: LibraryItemResources {
                thumbnail_url: "https://example.com/thumb.png".to_string(),
            },
        }),
        _ => None,
    }
}

// ── Entry-point ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> *mut Plugin {
    Box::into_raw(Box::new(PLUGIN))
}
