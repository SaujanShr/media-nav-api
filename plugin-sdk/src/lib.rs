use serde::Serialize;

/// Descriptive information about a plugin, shown in catalogues and UIs.
#[derive(Serialize)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    pub nsfw: bool,
}

/// Static assets the plugin ships — loaded by the frontend at runtime.
#[derive(Serialize)]
pub struct PluginResources {
    pub script_url: String,
    pub style_url: Option<String>,
    pub icon_url: String,
    pub banner_url: String,
}

/// The interface every plugin shared library must implement.
///
/// # Building a plugin
///
/// Create a `cdylib` crate that depends on `plugin-sdk`, implement
/// `PluginInstance`, and export a `create_plugin` symbol:
///
/// ```rust
/// #[no_mangle]
/// pub extern "C" fn create_plugin() -> *mut std::ffi::c_void {
///     let plugin: Box<dyn PluginInstance> = Box::new(MyPlugin::new());
///     Box::into_raw(Box::new(plugin)) as *mut std::ffi::c_void
/// }
/// ```
///
/// Place the compiled `.so` / `.dylib` in the directory pointed to by
/// `PLUGINS_DIR` and it will be picked up on the next server start.
pub trait PluginInstance: Send + Sync {
    fn id(&self) -> &str;
    fn version(&self) -> &str;
    fn metadata(&self) -> &PluginMetadata;
    fn resources(&self) -> &PluginResources;
}

