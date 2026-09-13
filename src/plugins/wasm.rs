use std::fmt;
use std::time::Duration;

use extism::{CompiledPlugin, Manifest, Plugin as ExtismPlugin, PluginBuilder, Pool, Wasm};
use serde::de::DeserializeOwned;
use serde::Serialize;

use plugin_sdk::library_item::LibraryItemDetail;
use plugin_sdk::plugin::{FetchRequest, FetchResult, PluginCallError, PluginInfo};
use plugin_sdk::query::schema::QuerySchema;

#[cfg(test)]
#[path = "tests/wasm.rs"]
mod tests;

// ── Config ────────────────────────────────────────────────────────────────────

const ALLOWED_HOSTS: &[&str] = &["localhost"];

const EXPORT_PLUGIN_INFO: &str = "plugin_info";
const EXPORT_SCHEMA:      &str = "schema";
const EXPORT_FETCH:       &str = "fetch";
const EXPORT_ENRICH:      &str = "enrich";

const CHECKOUT_TIMEOUT: Duration = Duration::from_secs(5);

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct CompileError(extism::Error);

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to compile plugin: {}", self.0)
    }
}

pub struct CallError {
    function: String,
    kind:     CallErrorKind,
}

enum CallErrorKind {
    Checkout(extism::Error),
    PoolExhausted,
    Serialize(serde_json::Error),
    Call(extism::Error),
    Deserialize(serde_json::Error),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = &self.function;
        match &self.kind {
            CallErrorKind::Checkout(e)    => write!(f, "failed to check out a plugin instance for \"{function}\": {e}"),
            CallErrorKind::PoolExhausted   => write!(f, "timed out waiting for a free plugin instance for \"{function}\""),
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
    pool: &Pool,
    function: &str,
    input: &I,
) -> Result<O, CallError> {
    call_kind(pool, function, input)
        .map_err(|kind| CallError { function: function.to_string(), kind })
}

fn call_kind<I: Serialize + ?Sized, O: DeserializeOwned>(
    pool: &Pool,
    function: &str,
    input: &I,
) -> Result<O, CallErrorKind> {
    let mut plugin = pool.get(CHECKOUT_TIMEOUT)
        .map_err(CallErrorKind::Checkout)?
        .ok_or(CallErrorKind::PoolExhausted)?;

    let input = serde_json::to_string(input)
        .map_err(CallErrorKind::Serialize)?;

    let output: String = plugin.call(function, input.as_str())
        .map_err(CallErrorKind::Call)?;

    serde_json::from_str(&output)
        .map_err(CallErrorKind::Deserialize)
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn compile(wasm: &[u8]) -> Result<Pool, CompileError> {
    let manifest = manifest_for(wasm);
    let compiled = CompiledPlugin::new(PluginBuilder::new(&manifest).with_wasi(true)).map_err(CompileError)?;

    Ok(Pool::new(move || ExtismPlugin::new_from_compiled(&compiled)))
}

pub fn plugin_info(pool: &Pool) -> Result<PluginInfo, CallError> {
    call(pool, EXPORT_PLUGIN_INFO, &())
}

pub fn schema(pool: &Pool) -> Result<QuerySchema, CallError> {
    call(pool, EXPORT_SCHEMA, &())
}

pub fn fetch(pool: &Pool, request: &FetchRequest) -> Result<Result<FetchResult, PluginCallError>, CallError> {
    call(pool, EXPORT_FETCH, request)
}

pub fn enrich(pool: &Pool, id: &str) -> Result<Result<Option<LibraryItemDetail>, PluginCallError>, CallError> {
    call(pool, EXPORT_ENRICH, id)
}
