mod enrich;
mod fetch;
mod media;
mod plugin;
mod schema;

use plugin_sdk::plugin::Plugin;

// ── Entry-point ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> *mut Plugin {
    Box::into_raw(Box::new(plugin::PLUGIN))
}
