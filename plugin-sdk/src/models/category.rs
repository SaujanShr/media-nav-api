use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Category {
    Audio,
    Book,
    Game,
    Image,
    Video,
    Unknown,
}
