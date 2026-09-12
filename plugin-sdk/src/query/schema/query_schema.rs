use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::query::schema::QueryFieldSchema;
use crate::query::Query;

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct QueryValidationError {
    pub field:   String,
    pub message: String,
}

impl QueryValidationError {
    fn new(field: &str, message: String) -> Self {
        Self { field: field.to_string(), message }
    }
}

// ── Schema ────────────────────────────────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize)]
pub struct QuerySchema {
    pub fields: HashMap<String, QueryFieldSchema>,
}

impl QuerySchema {
    pub fn validate(&self, query: &Query) -> Result<(), Vec<QueryValidationError>> {
        let errors: Vec<_> = self.fields
            .iter()
            .filter_map(|(name, field)| {
                field.validate_for(query, name)
                    .err()
                    .map(|msg| QueryValidationError::new(name, msg))
            })
            .collect();

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn default(&self) -> Query {
        let mut query = Query::default();
        for (name, field) in &self.fields {
            field.apply_default(name, &mut query);
        }
        query
    }
}
