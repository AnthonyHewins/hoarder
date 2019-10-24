use diesel::insert_into;
use diesel::result::{Error, QueryResult};
use diesel::prelude::*;

use crate::schema;
use super::TopLevelUpsert;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Publisher {
    pub id: i64,
    pub name: String
}

impl<'a> TopLevelUpsert<&'a str> for Publisher {
    fn upsert(conn: &PgConnection, s: &'a str) -> QueryResult<i64> {
        use schema::publishers::dsl::*;

        match publishers.select(id).filter(name.eq(s)).first::<i64>(conn) {
            Ok(existing_id) => Ok(existing_id),
            Err(e) => match e {
                Error::NotFound => Publisher::insert(conn, s),
                _ => Err(e)
            }
        }
    }

    fn insert(conn: &PgConnection, s: &'a str) -> QueryResult<i64> {
        use schema::publishers::dsl::*;

        insert_into(publishers)
            .values( name.eq(s) )
            .returning( id )
            .get_result::<i64>(conn)
    }
}
