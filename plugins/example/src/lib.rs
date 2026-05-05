use plugin_sdk::{PluginInstance, PluginMetadata, PluginResources};

// ── Plugin ────────────────────────────────────────────────────────────────────

struct ExamplePlugin {
    metadata: PluginMetadata,
    resources: PluginResources,
}

impl ExamplePlugin {
    fn new() -> Self {
        ExamplePlugin {
            metadata: PluginMetadata {
                name: "Example Plugin".to_string(),
                description: "A minimal example plugin for media-nav.".to_string(),
                category: "example".to_string(),
                nsfw: false,
            },
            resources: PluginResources {
                script_url: "/plugins/example/main.js".to_string(),
                style_url: Some("/plugins/example/style.css".to_string()),
                icon_url: "/plugins/example/icon.png".to_string(),
                banner_url: "/plugins/example/banner.png".to_string(),
            },
        }
    }
}

impl PluginInstance for ExamplePlugin {
    fn id(&self) -> &str {
        "example"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn resources(&self) -> &PluginResources {
        &self.resources
    }
}

// ── Entry-point ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> *mut std::ffi::c_void {
    let plugin: Box<dyn PluginInstance> = Box::new(ExamplePlugin::new());
    Box::into_raw(Box::new(plugin)) as *mut std::ffi::c_void
}


