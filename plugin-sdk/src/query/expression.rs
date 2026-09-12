use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize)]
pub struct SortOption {
    pub sort:       String,
    pub ascending:  bool,
    pub descending: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SortExpression {
    pub sort:      String,
    pub direction: SortDirection,
}

#[derive(Serialize, Deserialize)]
pub struct FilterExpression {
    #[serde(default)]
    pub include: HashSet<String>,
    #[serde(default)]
    pub exclude: HashSet<String>,
}
