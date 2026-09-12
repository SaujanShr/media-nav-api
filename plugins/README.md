# Plugins

Sandboxed WebAssembly modules that extend the API with new content sources. Each plugin provides
a query schema, fetch logic, and enrichment for media items, and runs through `extism`/`wasmtime` —
not `dlopen`'d native code. The host talks to a plugin entirely over JSON via four exports:
`plugin_info`, `schema`, `fetch`, and `enrich`.

## Creating a Plugin

1. **Create a standalone crate** (plugins are *not* members of the root workspace — they target
   `wasm32-wasip1`, not the host's native arch):
   ```sh
   cd plugins
   cargo new --lib my-plugin
   ```

2. **Configure `plugins/my-plugin/Cargo.toml`:**
   ```toml
   [workspace]

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   plugin-sdk = { path = "../../plugin-sdk", features = ["guest"] }
   extism-pdk = "1.4"
   serde = { version = "1.0", features = ["derive"] }
   ```

3. **Implement the four exports** (see `plugins/example/src/` for a full reference):
   ```rust
   use extism_pdk::{plugin_fn, FnResult, Json};
   use plugin_sdk::plugin::{PluginInfo, PluginMetadata, PluginResources};
   use plugin_sdk::query::schema::QuerySchema;

   #[plugin_fn]
   pub fn plugin_info() -> FnResult<Json<PluginInfo>> {
       Ok(Json(PluginInfo {
           id:      "my-plugin".into(),
           version: "0.1.0".into(),
           metadata:  PluginMetadata { name: "My Plugin".into(), description: "...".into(), nsfw: false },
           resources: PluginResources { icon_url: "...".into(), banner_url: "...".into() },
       }))
   }

   #[plugin_fn]
   pub fn schema() -> FnResult<Json<QuerySchema>> { /* declare queryable fields */ }

   #[plugin_fn]
   pub fn fetch(req: Json<plugin_sdk::plugin::FetchRequest>) -> FnResult<Json<Result<plugin_sdk::plugin::FetchResult, plugin_sdk::plugin::PluginCallError>>> { /* ... */ }

   #[plugin_fn]
   pub fn enrich(id: Json<String>) -> FnResult<Json<Result<Option<plugin_sdk::library_item::LibraryItemDetail>, plugin_sdk::plugin::PluginCallError>>> { /* ... */ }
   ```

4. **Network access is denied by default.** A plugin can only reach hosts listed in
   `ALLOWED_HOSTS` in `src/plugins/wasm.rs` on the host side, via `extism_pdk::http::request` —
   there's no raw socket access. Add your provider's host there if it isn't already covered.

5. **Update `plugins/Makefile`** to add build/install targets for your plugin (follow the
   `example` pattern — build with `--target wasm32-wasip1`, copy the resulting `.wasm` into
   `plugins/`).

6. **Build and test:**
   ```sh
   make build-plugins    # from repository root
   # Restart API server to load new plugin
   ```

## Structure

```
plugins/example/
├── Cargo.toml       # standalone workspace, wasm32-wasip1 target
└── src/
    ├── lib.rs        # entry points: plugin_info, schema, fetch, enrich
    ├── schema.rs      # query schema (search/filter/sort fields)
    ├── fetch.rs       # fetch library items from provider
    └── enrich.rs      # enrich item details from provider
```

## Tips

- **Reference:** Use `plugins/example/` as template
- **SDK:** See `plugin-sdk/src/` for models and types — everything crossing the host↔guest
  boundary must be `Serialize + Deserialize` (no `fn` pointers, no `&'static` borrows)
- **Provider communication:** Use `extism_pdk::http::request`, not a native HTTP client — the
  guest has no socket access outside what the host's manifest allows
- **Errors:** Return `None` or empty results, don't panic
- **Reloading:** Restart API server after rebuilding
