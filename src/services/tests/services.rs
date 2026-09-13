use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;

use sqlx::error::{DatabaseError, ErrorKind};

use super::is_duplicate_key;

#[derive(Debug)]
struct FakeDbError {
    code: Option<String>,
}

impl fmt::Display for FakeDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fake db error")
    }
}

impl StdError for FakeDbError {}

impl DatabaseError for FakeDbError {
    fn message(&self) -> &str {
        "fake db error"
    }

    fn code(&self) -> Option<Cow<'_, str>> {
        self.code.as_deref().map(Cow::Borrowed)
    }

    fn as_error(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

#[test]
fn detects_a_unique_violation_by_its_postgres_code() {
    let err = sqlx::Error::Database(Box::new(FakeDbError { code: Some("23505".to_string()) }));
    assert!(is_duplicate_key(&err));
}

#[test]
fn ignores_other_postgres_error_codes() {
    let err = sqlx::Error::Database(Box::new(FakeDbError { code: Some("23503".to_string()) }));
    assert!(!is_duplicate_key(&err));
}

#[test]
fn ignores_database_errors_with_no_code() {
    let err = sqlx::Error::Database(Box::new(FakeDbError { code: None }));
    assert!(!is_duplicate_key(&err));
}

#[test]
fn ignores_non_database_errors() {
    assert!(!is_duplicate_key(&sqlx::Error::RowNotFound));
}
