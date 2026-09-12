use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BooleanFieldSchema {
    pub default: bool,
}
