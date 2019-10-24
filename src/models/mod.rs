use diesel::prelude::*;
use diesel::result::QueryResult;

// User/enterprise information
pub mod domain;
pub mod employee;
pub mod machine;

// Software info
pub mod publisher;
pub mod program;

// Audit data
pub mod sw_report;
pub mod machines_program;

pub trait TopLevelUpsert<T> {
    fn upsert(conn: &PgConnection, _: T) -> QueryResult<i64>;
    fn insert(conn: &PgConnection, _: T) -> QueryResult<i64>;
}
