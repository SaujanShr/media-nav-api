use plugin_sdk::category::Category;
use plugin_sdk::plugin::{Plugin, PluginMetadata, PluginResources};

use crate::{enrich, fetch, media, schema};

pub const PLUGIN: Plugin = Plugin {
    id: "anime",
    version: "0.1.0",
    metadata: PluginMetadata {
        name: "Anime",
        description: "Browse and search anime via Jikan (MyAnimeList).",
        category: Category::Watch,
        nsfw: false,
    },
    resources: PluginResources {
        icon_url: "https://cdn.myanimelist.net/img/sp/icon/apple-touch-icon-256.png",
        banner_url: "https://image.myanimelist.net/ui/OK6W_koKDTOqqqLDbIoPAq4SZ1amR6RJ6R1z-GcZwEo",
    },
    schema:    schema::schema,
    fetch:     fetch::fetch,
    enrich:    enrich::enrich,
    media:     media::media,
};
