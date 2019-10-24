use diesel::insert_into;
use diesel::result::{Error, QueryResult};
use diesel::prelude::*;

use crate::schema;
use super::TopLevelUpsert;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Employee {
    pub id: i64,
    pub name: String
}

impl<'a> TopLevelUpsert<&'a str> for Employee {
    fn upsert(conn: &PgConnection, s: &'a str) -> QueryResult<i64> {
        use schema::employees::dsl::*;

        match employees.select(id).filter(name.eq(s)).first::<i64>(conn) {
            Ok(existing_id) => Ok(existing_id),
            Err(e) => match e {
                Error::NotFound => Employee::insert(conn, s),
                _ => Err(e)
            }
        }
    }

    fn insert(conn: &PgConnection, s: &'a str) -> QueryResult<i64> {
        use schema::employees::dsl::*;

        insert_into(employees)
            .values( name.eq(s) )
            .returning( id )
            .get_result::<i64>(conn)
    }
}
