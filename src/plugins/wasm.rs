use std::fmt;

use extism::{Manifest, Plugin as ExtismPlugin, Wasm};
use serde::de::DeserializeOwned;
use serde::Serialize;

use plugin_sdk::library_item::LibraryItemDetail;
use plugin_sdk::plugin::{FetchRequest, FetchResult, PluginCallError, PluginInfo};
use plugin_sdk::query::schema::QuerySchema;

// ── Config ────────────────────────────────────────────────────────────────────

const ALLOWED_HOSTS: &[&str] = &["localhost"];

const EXPORT_PLUGIN_INFO: &str = "plugin_info";
const EXPORT_SCHEMA:      &str = "schema";
const EXPORT_FETCH:       &str = "fetch";
const EXPORT_ENRICH:      &str = "enrich";

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct CallError {
    function: String,
    kind:     CallErrorKind,
}

enum CallErrorKind {
    Instantiate(extism::Error),
    Serialize(serde_json::Error),
    Call(extism::Error),
    Deserialize(serde_json::Error),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = &self.function;
        
        match &self.kind {
            CallErrorKind::Instantiate(e) => write!(f, "failed to instantiate plugin for \"{function}\": {e}"),
            CallErrorKind::Serialize(e)   => write!(f, "failed to serialize input for \"{function}\": {e}"),
            CallErrorKind::Call(e)        => write!(f, "call to \"{function}\" failed: {e}"),
            CallErrorKind::Deserialize(e) => write!(f, "failed to deserialize output of \"{function}\": {e}"),
        }
    }
}

// ── Private ───────────────────────────────────────────────────────────────────

fn manifest_for(wasm: &[u8]) -> Manifest {
    Manifest::new([Wasm::data(wasm.to_vec())])
        .with_allowed_hosts(ALLOWED_HOSTS.iter().map(|h| h.to_string()))
}

fn call<I: Serialize + ?Sized, O: DeserializeOwned>(
    wasm: &[u8],
    function: &str,
    input: &I,
) -> Result<O, CallError> {
    call_kind(wasm, function, input)
    .map_err(|kind| CallError { function: function.to_string(), kind })
}

fn call_kind<I: Serialize + ?Sized, O: DeserializeOwned>(
    wasm: &[u8],
    function: &str,
    input: &I,
) -> Result<O, CallErrorKind> {
    let manifest = manifest_for(wasm);
    let mut plugin = ExtismPlugin::new(&manifest, [], true).map_err(CallErrorKind::Instantiate)?;

    let input = serde_json::to_string(input).map_err(CallErrorKind::Serialize)?;
    let output = plugin.call::<&str, &str>(function, &input).map_err(CallErrorKind::Call)?;

    serde_json::from_str(output).map_err(CallErrorKind::Deserialize)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn plugin_info(wasm: &[u8]) -> Result<PluginInfo, CallError> {
    call(wasm, EXPORT_PLUGIN_INFO, &())
}

pub fn schema(wasm: &[u8]) -> Result<QuerySchema, CallError> {
    call(wasm, EXPORT_SCHEMA, &())
}

pub fn fetch(wasm: &[u8], request: &FetchRequest) -> Result<Result<FetchResult, PluginCallError>, CallError> {
    call(wasm, EXPORT_FETCH, request)
}

pub fn enrich(wasm: &[u8], id: &str) -> Result<Result<Option<LibraryItemDetail>, PluginCallError>, CallError> {
    call(wasm, EXPORT_ENRICH, id)
}
