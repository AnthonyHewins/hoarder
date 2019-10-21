use diesel::insert_into;
use diesel::result::Error as DbError;
use diesel::prelude::*;

use chrono::NaiveDate;

use crate::schema;

#[derive(Queryable, Serialize, Deserialize)]
struct SwReport {
    id: i64,
    generation_date: NaiveDate
}

impl SwReport {
    pub fn upsert(conn: &PgConnection, date: NaiveDate) -> Result<i64, DbError> {
        use schema::sw_reports::dsl::*;

        insert_into(sw_reports)
            .values( date )
            .returning( id )
            .get_result(conn)?
    }
}
