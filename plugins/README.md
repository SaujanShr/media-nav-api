# Plugins

Dynamically-loaded Rust libraries that extend the API with new content sources. Each plugin provides schema definitions, fetch logic, and enrichment for media items.

## Building

```sh
# From repository root
make build-plugins      # compile and install all plugins

# Or from plugins/ directory
cd plugins
make build              # compile all plugin dylibs
make install            # copy to plugins/ and sign
make clean              # remove installed plugins

# Individual plugin
make build-example
make install-example
```

## Creating a Plugin

1. **Create workspace member:**
   ```sh
   cd plugins
   cargo new --lib my-plugin
   ```

2. **Configure `plugins/my-plugin/Cargo.toml`:**
   ```toml
   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   plugin-sdk = { path = "../../plugin-sdk" }
   ```

3. **Add to root `Cargo.toml`:**
   ```toml
   [workspace]
   members = ["plugins/my-plugin", ...]
   ```

4. **Implement plugin interface** (see `plugins/example/src/` for structure):
   ```rust
   use plugin_sdk::plugin::{Plugin, PluginMetadata, PluginResources};

   pub const PLUGIN: Plugin = Plugin {
       id: "my-plugin",
       version: "0.1.0",
       metadata: PluginMetadata {
           name: "My Plugin",
           description: "What this plugin provides",
           nsfw: false,
       },
       resources: PluginResources {
           icon_url: "https://example.com/icon.png",
           banner_url: "https://example.com/banner.png",
       },
       schema: schema::schema,    // query field definitions
       fetch: fetch::fetch,       // paginated library items
       enrich: enrich::enrich,    // detailed item metadata
   };

   #[no_mangle]
   pub extern "C" fn _plugin_create() -> *const Plugin {
       &PLUGIN
   }
   ```

5. **Update `plugins/Makefile`** to include your new plugin targets (follow the example pattern).

6. **Build and test:**
   ```sh
   make build-plugins    # from repository root
   # Restart API server to load new plugin
   ```

## Structure

```
plugins/example/
├── Cargo.toml
└── src/
    ├── lib.rs       # entry point, exports _plugin_create
    ├── plugin.rs    # Plugin constant and metadata
    ├── schema.rs    # query schema (search/filter/sort fields)
    ├── fetch.rs     # fetch library items from provider
    └── enrich.rs    # enrich item details from provider
```

## Tips

- **Reference:** Use `plugins/example/` as template
- **SDK:** See `plugin-sdk/src/` for models and types
- **Provider communication:** Use `reqwest` to call provider HTTP APIs
- **Errors:** Return `None` or empty results, don't panic
- **Reloading:** Restart API server after rebuilding
