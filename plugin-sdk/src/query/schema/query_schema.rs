use std::collections::HashMap;

use crate::query::expression::{FilterExpression, SortExpression};
use crate::query::schema::QueryFieldSchema;
use crate::query::Query;

pub struct QueryValidationError {
    pub field:   String,
    pub message: String,
}

pub struct QuerySchema {
    pub fields: HashMap<&'static str, QueryFieldSchema>,
}

impl QuerySchema {
    pub fn validate(&self, query: &Query) -> Result<(), Vec<QueryValidationError>> {
        let errors: Vec<QueryValidationError> = self.fields
            .iter()
            .filter_map(|(name, field)| {
                let key = *name;
                let result = match field {
                    QueryFieldSchema::Search(f) =>
                        f.validate(query.search_fields.get(key).map(String::as_str)),
                    QueryFieldSchema::Boolean(_) =>
                        Ok(()),
                    QueryFieldSchema::Number(f) =>
                        f.validate(query.number_fields.get(key).copied()),
                    QueryFieldSchema::Date(f) =>
                        f.validate(query.date_fields.get(key).copied()),
                    QueryFieldSchema::Filter(f) =>
                        f.validate(query.filter_fields.get(key)),
                    QueryFieldSchema::Sort(f) =>
                        f.validate(query.sort_fields.get(key)),
                };
                result.err().map(|message| QueryValidationError {
                    field: key.to_string(),
                    message,
                })
            })
            .collect();

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn default(&self) -> Query {
        let mut query = Query::default();

        for (name, field) in &self.fields {
            let key = name.to_string();
            match field {
                QueryFieldSchema::Search(f) => {
                    if let Some(v) = &f.default {
                        query.search_fields.insert(key, v.clone());
                    }
                }
                QueryFieldSchema::Boolean(f) => {
                    query.boolean_fields.insert(key, f.default);
                }
                QueryFieldSchema::Number(f) => {
                    if f.default_from.is_some() || f.default_to.is_some() {
                        query.number_fields.insert(key, (f.default_from, f.default_to));
                    }
                }
                QueryFieldSchema::Date(f) => {
                    if f.default_from.is_some() || f.default_to.is_some() {
                        query.date_fields.insert(key, (f.default_from, f.default_to));
                    }
                }
                QueryFieldSchema::Filter(f) => {
                    if f.default_include.is_some() || f.default_exclude.is_some() {
                        query.filter_fields.insert(key, FilterExpression {
                            include: f.default_include.clone().unwrap_or_default(),
                            exclude: f.default_exclude.clone().unwrap_or_default(),
                        });
                    }
                }
                QueryFieldSchema::Sort(f) => {
                    query.sort_fields.insert(key, SortExpression {
                        sort:      f.default_sort.clone(),
                        direction: f.default_direction,
                    });
                }
            }
        }

        query
    }
}
