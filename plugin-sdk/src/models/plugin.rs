use std::fmt;

use serde::{Deserialize, Serialize};

use crate::library_item::LibraryItem;
use crate::query::Query;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginCallError {
    pub status:  u16,
    pub message: String,
}

impl PluginCallError {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self { status, message: message.into() }
    }
}

impl fmt::Display for PluginCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FetchRequest {
    pub page:      u32,
    pub page_size: u32,
    pub query:     Query,
}

#[derive(Serialize, Deserialize)]
pub struct FetchResult {
    pub items: Vec<LibraryItem>,
    pub total: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name:        String,
    pub description: String,
    pub nsfw:        bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PluginResources {
    pub icon_url:   String,
    pub banner_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id:        String,
    pub version:   String,
    pub metadata:  PluginMetadata,
    pub resources: PluginResources,
}
