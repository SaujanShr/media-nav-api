mod field;
mod query_schema;

pub use field::{
    BooleanFieldSchema, DateFieldSchema, FilterFieldSchema,
    NumberFieldSchema, SearchFieldSchema, SortFieldSchema,
};
pub use query_schema::{QuerySchema, QueryValidationError};

pub enum QueryFieldSchema {
    Search(SearchFieldSchema),
    Boolean(BooleanFieldSchema),
    Number(NumberFieldSchema),
    Date(DateFieldSchema),
    Filter(FilterFieldSchema),
    Sort(SortFieldSchema),
}
