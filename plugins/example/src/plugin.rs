use plugin_sdk::category::Category;
use plugin_sdk::plugin::{Plugin, PluginMetadata, PluginResources};

use crate::{enrich, fetch, media, schema};

pub const PLUGIN: Plugin = Plugin {
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
    schema:    schema::schema,
    fetch:     fetch::fetch,
    enrich:    enrich::enrich,
    media:     media::media,
};

