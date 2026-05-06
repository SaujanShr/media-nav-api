mod models;
pub mod query;

pub use models::category;
pub use models::library_item;
pub use models::media;
pub use models::plugin;

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
