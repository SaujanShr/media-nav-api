pub const USERNAME_MIN_LENGTH: usize = 3;
pub const USERNAME_MAX_LENGTH: usize = 50;
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;
pub const PLAYLIST_NAME_MIN_LENGTH: usize = 1;
pub const PLAYLIST_NAME_MAX_LENGTH: usize = 200;

pub fn validate_username(username: &str) -> Result<(), String> {
    let len = username.len();
    if len < USERNAME_MIN_LENGTH {
        return Err(format!("Username must be at least {} characters", USERNAME_MIN_LENGTH));
    }
    if len > USERNAME_MAX_LENGTH {
        return Err(format!("Username must not exceed {} characters", USERNAME_MAX_LENGTH));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("Username can only contain letters, numbers, underscores, and hyphens".into());
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), String> {
    let len = password.len();
    if len < PASSWORD_MIN_LENGTH {
        return Err(format!("Password must be at least {} characters", PASSWORD_MIN_LENGTH));
    }
    if len > PASSWORD_MAX_LENGTH {
        return Err(format!("Password must not exceed {} characters", PASSWORD_MAX_LENGTH));
    }
    Ok(())
}

pub fn validate_playlist_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    let len = trimmed.len();
    if len < PLAYLIST_NAME_MIN_LENGTH {
        return Err("Playlist name cannot be empty".into());
    }
    if len > PLAYLIST_NAME_MAX_LENGTH {
        return Err(format!("Playlist name must not exceed {} characters", PLAYLIST_NAME_MAX_LENGTH));
    }
    Ok(())
}
