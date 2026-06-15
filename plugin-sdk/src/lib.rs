#[macro_export]
macro_rules! push_attr {
    ($attrs:expr, $label:expr, $opt:expr) => {
        if let Some(v) = $opt {
            $attrs.push($crate::library_item::LibraryItemAttribute {
                label: $label.into(),
                value: v.to_string(),
            });
        }
    };
}

mod models;
pub mod partial_date;
pub mod query;
pub mod utils;

pub use models::library_item;
pub use models::media_item;
pub use models::plugin;
