mod enrich;
mod fetch;
mod schema;

use extism_pdk::{plugin_fn, FnResult, Json};

use plugin_sdk::plugin::{PluginCallError, PluginInfo, PluginMetadata, PluginResources};
use plugin_sdk::query::schema::QuerySchema;

// ── Entry-points ──────────────────────────────────────────────────────────────

#[plugin_fn]
pub fn plugin_info() -> FnResult<Json<PluginInfo>> {
    Ok(Json(PluginInfo {
        id:      "example".into(),
        version: "0.1.0".into(),
        metadata: PluginMetadata {
            name:        "Example Plugin".into(),
            description: "A minimal example plugin for media-nav.".into(),
            nsfw:        false,
        },
        resources: PluginResources {
            icon_url:   "https://example.com/icon.png".into(),
            banner_url: "https://example.com/banner.png".into(),
        },
    }))
}

#[plugin_fn]
pub fn schema() -> FnResult<Json<QuerySchema>> {
    Ok(Json(schema::schema()))
}

#[plugin_fn]
pub fn fetch(req: Json<plugin_sdk::plugin::FetchRequest>) -> FnResult<Json<Result<plugin_sdk::plugin::FetchResult, PluginCallError>>> {
    Ok(Json(fetch::fetch(req.into_inner())))
}

#[plugin_fn]
pub fn enrich(id: Json<String>) -> FnResult<Json<Result<Option<plugin_sdk::library_item::LibraryItemDetail>, PluginCallError>>> {
    Ok(Json(enrich::enrich(&id.into_inner())))
}
