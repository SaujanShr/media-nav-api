use super::PluginCallError;

#[test]
fn display_includes_status_and_message() {
    let err = PluginCallError::new(502, "provider unreachable");
    assert_eq!(err.to_string(), "502: provider unreachable");
}
