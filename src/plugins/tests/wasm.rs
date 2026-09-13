use super::{CallError, CallErrorKind};

fn error(kind: CallErrorKind) -> CallError {
    CallError { function: "fetch".to_string(), kind }
}

#[test]
fn displays_a_checkout_failure() {
    let err = error(CallErrorKind::Checkout(extism::Error::msg("pool closed")));
    assert_eq!(err.to_string(), "failed to check out a plugin instance for \"fetch\": pool closed");
}

#[test]
fn displays_a_pool_exhausted_error() {
    let err = error(CallErrorKind::PoolExhausted);
    assert_eq!(err.to_string(), "timed out waiting for a free plugin instance for \"fetch\"");
}

#[test]
fn displays_a_serialize_failure() {
    let serialize_err = serde_json::from_str::<i32>("not json").unwrap_err();
    let err = error(CallErrorKind::Serialize(serialize_err));
    assert!(err.to_string().starts_with("failed to serialize input for \"fetch\": "));
}

#[test]
fn displays_a_call_failure() {
    let err = error(CallErrorKind::Call(extism::Error::msg("guest trapped")));
    assert_eq!(err.to_string(), "call to \"fetch\" failed: guest trapped");
}

#[test]
fn displays_a_deserialize_failure() {
    let deserialize_err = serde_json::from_str::<i32>("not json").unwrap_err();
    let err = error(CallErrorKind::Deserialize(deserialize_err));
    assert!(err.to_string().starts_with("failed to deserialize output of \"fetch\": "));
}
