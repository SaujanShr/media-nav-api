use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BooleanFieldSchema {
    pub default: bool,
}
