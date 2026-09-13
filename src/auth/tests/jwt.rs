use super::{create_token, decoding_key, validate_token, validation, TOKEN_EXPIRY_SECS};

#[test]
fn create_and_validate_round_trip() {
    let token = create_token("user-1", "secret").unwrap();
    let claims = validate_token(&token, &decoding_key("secret"), &validation()).unwrap();

    assert_eq!(claims.sub, "user-1");
    assert_eq!(claims.exp, claims.iat + TOKEN_EXPIRY_SECS);
}

#[test]
fn validate_token_rejects_a_token_signed_with_a_different_secret() {
    let token = create_token("user-1", "secret").unwrap();
    let result = validate_token(&token, &decoding_key("wrong-secret"), &validation());

    assert!(result.is_err());
}

#[test]
fn validate_token_rejects_a_malformed_token() {
    let result = validate_token("not-a-jwt", &decoding_key("secret"), &validation());

    assert!(result.is_err());
}
