use super::{encode_query_param, filename_from_url};
use crate::library_item::LibraryItemAttribute;

#[test]
fn filename_from_url_takes_the_last_path_segment() {
    assert_eq!(filename_from_url("https://example.com/a/b/photo.png"), "photo.png");
}

#[test]
fn filename_from_url_returns_the_whole_string_when_there_is_no_slash() {
    assert_eq!(filename_from_url("photo.png"), "photo.png");
}

#[test]
fn encode_query_param_percent_encodes_special_characters() {
    assert_eq!(encode_query_param("a b&c"), "a%20b%26c");
}

#[test]
fn push_attr_pushes_when_some() {
    let mut attrs = Vec::new();
    crate::push_attr!(attrs, "Year", Some(2024));

    assert_eq!(attrs.len(), 1);
    assert_eq!(attrs[0].label, "Year");
    assert_eq!(attrs[0].value, "2024");
}

#[test]
fn push_attr_skips_when_none() {
    let mut attrs: Vec<LibraryItemAttribute> = Vec::new();
    crate::push_attr!(attrs, "Year", None::<i32>);

    assert!(attrs.is_empty());
}
