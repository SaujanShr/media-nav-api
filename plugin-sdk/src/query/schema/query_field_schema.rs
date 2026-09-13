use serde::{Deserialize, Serialize};

use crate::query::expression::{FilterExpression, SortExpression};
use crate::query::schema::{
    BooleanFieldSchema, DateFieldSchema, FilterFieldSchema,
    NumberFieldSchema, SearchFieldSchema, SortFieldSchema,
};
use crate::query::Query;

#[cfg(test)]
#[path = "tests/query_field_schema.rs"]
mod tests;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryFieldSchema {
    Search(SearchFieldSchema),
    Boolean(BooleanFieldSchema),
    Number(NumberFieldSchema),
    Date(DateFieldSchema),
    Filter(FilterFieldSchema),
    Sort(SortFieldSchema),
}

impl QueryFieldSchema {
    pub fn validate_for(&self, query: &Query, key: &str) -> Result<(), String> {
        match self {
            Self::Search(f)  => f.validate(query.search_fields.get(key).map(String::as_str)),
            Self::Boolean(_) => Ok(()),
            Self::Number(f)  => f.validate(query.number_fields.get(key).copied()),
            Self::Date(f)    => f.validate(query.date_fields.get(key).copied()),
            Self::Filter(f)  => f.validate(query.filter_fields.get(key)),
            Self::Sort(f)    => f.validate(query.sort_fields.get(key)),
        }
    }

    pub fn apply_default(&self, key: &str, query: &mut Query) {
        match self {
            Self::Search(f) => {
                if let Some(v) = &f.default {
                    query.search_fields.insert(key.to_string(), v.clone());
                }
            }
            Self::Boolean(f) => {
                query.boolean_fields.insert(key.to_string(), f.default);
            }
            Self::Number(f) => {
                if f.default_from.is_some() || f.default_to.is_some() {
                    query.number_fields.insert(key.to_string(), (f.default_from, f.default_to));
                }
            }
            Self::Date(f) => {
                if f.default_from.is_some() || f.default_to.is_some() {
                    query.date_fields.insert(key.to_string(), (f.default_from, f.default_to));
                }
            }
            Self::Filter(f) => {
                if f.default_include.is_some() || f.default_exclude.is_some() {
                    query.filter_fields.insert(key.to_string(), FilterExpression {
                        include: f.default_include.clone().unwrap_or_default(),
                        exclude: f.default_exclude.clone().unwrap_or_default(),
                    });
                }
            }
            Self::Sort(f) => {
                query.sort_fields.insert(key.to_string(), SortExpression {
                    sort:      f.default_sort.clone(),
                    direction: f.default_direction,
                });
            }
        }
    }
}
