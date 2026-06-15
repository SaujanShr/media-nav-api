use plugin_sdk::plugin::{Plugin, PluginMetadata, PluginResources};

use crate::{enrich, fetch, schema};

pub const PLUGIN: Plugin = Plugin {
    id:      "example",
    version: "0.1.0",
    metadata: PluginMetadata {
        name:        "Example Plugin",
        description: "A minimal example plugin for media-nav.",
        nsfw:        false,
    },
    resources: PluginResources {
        icon_url:   "https://example.com/icon.png",
        banner_url: "https://example.com/banner.png",
    },
    schema:    schema::schema,
    fetch:     fetch::fetch,
    enrich:    enrich::enrich,
};

