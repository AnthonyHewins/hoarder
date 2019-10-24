use diesel::insert_into;
use diesel::result::{Error, QueryResult};
use diesel::prelude::*;

use chrono::NaiveDate;

use crate::schema;
use super::TopLevelUpsert;

#[derive(Queryable, Serialize, Deserialize)]
pub struct SwReport {
    id: i64,
    generation_date: NaiveDate
}

impl TopLevelUpsert<NaiveDate> for SwReport {
    fn upsert(conn: &PgConnection, date: NaiveDate) -> QueryResult<i64> {
        use schema::sw_reports::dsl::*;

        match sw_reports.select(id).filter(generation_date.eq(date)).first::<i64>(conn) {
            Ok(existing_id) => Ok(existing_id),
            Err(e) => match e {
                Error::NotFound => SwReport::insert(conn, date),
                _ => Err(e)
            }
        }
    }

    fn insert(conn: &PgConnection, date: NaiveDate) -> QueryResult<i64> {
        use schema::sw_reports::dsl::*;

        insert_into(sw_reports)
            .values( generation_date.eq(date) )
            .returning( id )
            .get_result::<i64>(conn)
    }
}
