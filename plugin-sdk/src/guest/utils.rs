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

pub fn filename_from_url(url: &str) -> String {
    url.rsplit('/').next().unwrap_or(url).to_string()
}

pub fn encode_query_param(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}
