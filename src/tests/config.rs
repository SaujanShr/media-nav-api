use std::sync::{Mutex, MutexGuard, OnceLock};

use super::Config;

const VARS: &[&str] = &["DATABASE_URL", "JWT_SECRET", "PLUGINS_DIR", "HOST", "PORT", "DB_MAX_CONNECTIONS"];

fn lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

// These tests all mutate the same process-wide env vars, and #[test] fns run
// concurrently by default, so each guard holds a lock for its whole lifetime
// to serialize them. Poisoning is recovered from so one panicking test doesn't
// wedge the rest.
struct EnvGuard(#[allow(dead_code)] MutexGuard<'static, ()>);

impl EnvGuard {
    fn set(vars: &[(&str, &str)]) -> Self {
        let guard = lock().lock().unwrap_or_else(|e| e.into_inner());
        for (key, value) in vars {
            unsafe { std::env::set_var(key, value) };
        }
        EnvGuard(guard)
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for key in VARS {
            unsafe { std::env::remove_var(key) };
        }
    }
}

#[test]
fn from_env_reads_every_field() {
    let _guard = EnvGuard::set(&[
        ("DATABASE_URL", "postgres://localhost/test"),
        ("JWT_SECRET", "a-secret"),
        ("PLUGINS_DIR", "./plugins"),
        ("HOST", "127.0.0.1"),
        ("PORT", "8080"),
        ("DB_MAX_CONNECTIONS", "20"),
    ]);

    let config = Config::from_env();

    assert_eq!(config.database_url, "postgres://localhost/test");
    assert_eq!(config.jwt_secret, "a-secret");
    assert_eq!(config.plugins_dir, "./plugins");
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
    assert_eq!(config.db_max_connections, 20);
}

#[test]
#[should_panic(expected = "DATABASE_URL must be set")]
fn from_env_panics_when_database_url_is_missing() {
    let _guard = EnvGuard::set(&[
        ("JWT_SECRET", "a-secret"),
        ("PLUGINS_DIR", "./plugins"),
        ("HOST", "127.0.0.1"),
        ("PORT", "8080"),
        ("DB_MAX_CONNECTIONS", "20"),
    ]);
    unsafe { std::env::remove_var("DATABASE_URL") };

    Config::from_env();
}

#[test]
#[should_panic(expected = "PORT must be a valid number")]
fn from_env_panics_when_port_is_not_a_number() {
    let _guard = EnvGuard::set(&[
        ("DATABASE_URL", "postgres://localhost/test"),
        ("JWT_SECRET", "a-secret"),
        ("PLUGINS_DIR", "./plugins"),
        ("HOST", "127.0.0.1"),
        ("PORT", "not-a-number"),
        ("DB_MAX_CONNECTIONS", "20"),
    ]);

    Config::from_env();
}

#[test]
#[should_panic(expected = "DB_MAX_CONNECTIONS must be a valid number")]
fn from_env_panics_when_db_max_connections_is_not_a_number() {
    let _guard = EnvGuard::set(&[
        ("DATABASE_URL", "postgres://localhost/test"),
        ("JWT_SECRET", "a-secret"),
        ("PLUGINS_DIR", "./plugins"),
        ("HOST", "127.0.0.1"),
        ("PORT", "8080"),
        ("DB_MAX_CONNECTIONS", "not-a-number"),
    ]);

    Config::from_env();
}
