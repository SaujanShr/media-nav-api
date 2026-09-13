use super::{
    validate_password, validate_playlist_name, validate_username,
    PASSWORD_MAX_LENGTH, PLAYLIST_NAME_MAX_LENGTH, USERNAME_MAX_LENGTH,
};

#[test]
fn validate_username_rejects_too_short() {
    assert!(validate_username("ab").is_err());
}

#[test]
fn validate_username_rejects_too_long() {
    assert!(validate_username(&"a".repeat(USERNAME_MAX_LENGTH + 1)).is_err());
}

#[test]
fn validate_username_rejects_invalid_characters() {
    assert!(validate_username("bad user!").is_err());
}

#[test]
fn validate_username_accepts_letters_numbers_underscore_and_hyphen() {
    assert!(validate_username("valid_user-123").is_ok());
}

#[test]
fn validate_password_rejects_too_short() {
    assert!(validate_password("short").is_err());
}

#[test]
fn validate_password_rejects_too_long() {
    assert!(validate_password(&"a".repeat(PASSWORD_MAX_LENGTH + 1)).is_err());
}

#[test]
fn validate_password_accepts_valid_length() {
    assert!(validate_password("a_valid_password").is_ok());
}

#[test]
fn validate_playlist_name_rejects_blank() {
    assert!(validate_playlist_name("   ").is_err());
}

#[test]
fn validate_playlist_name_rejects_too_long() {
    assert!(validate_playlist_name(&"a".repeat(PLAYLIST_NAME_MAX_LENGTH + 1)).is_err());
}

#[test]
fn validate_playlist_name_accepts_valid_name() {
    assert!(validate_playlist_name("My Playlist").is_ok());
}
