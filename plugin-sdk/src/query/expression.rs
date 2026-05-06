use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortDirection {
    Asc,
    Desc,
}

pub struct SortOption {
    pub sort:       String,
    pub ascending:  bool,
    pub descending: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SortExpression {
    pub sort:      String,
    pub direction: SortDirection,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FilterExpression {
    #[serde(default)]
    pub include: HashSet<String>,
    #[serde(default)]
    pub exclude: HashSet<String>,
}
