use std::collections::HashSet;

use crate::query::expression::FilterExpression;

pub struct FilterFieldSchema {
    pub supported:       HashSet<String>,
    pub multiple:        bool,
    pub include:         bool,
    pub exclude:         bool,
    pub default_include: Option<HashSet<String>>,
    pub default_exclude: Option<HashSet<String>>,
}

impl FilterFieldSchema {

    // ── Public ────────────────────────────────────────────────────────────────

    pub fn validate(&self, value: Option<&FilterExpression>) -> Result<(), String> {
        match value {
            None => Ok(()),
            Some(expr) => self.validate_filter(expr),
        }
    }

    // ── Private ───────────────────────────────────────────────────────────────

    fn validate_filter(&self, expr: &FilterExpression) -> Result<(), String> {
        let all: HashSet<&String> = expr.include.union(&expr.exclude).collect();

        let unsupported: Vec<&str> = all
            .into_iter()
            .filter(|s| !self.supported.contains(*s))
            .map(String::as_str)
            .collect();

        if !unsupported.is_empty() {
            return Err(format!("Unsupported selections: {}", unsupported.join(", ")))
        }
        Ok(())
    }
}
