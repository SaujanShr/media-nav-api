macro_rules! assert_ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return e.error_response(),
        }
    };
}

pub mod auth;
pub mod health;
pub mod library_item;
pub mod media;
pub mod playlist;
pub mod plugin;
pub mod settings;
