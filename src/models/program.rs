use diesel::PgConnection;
use diesel::insert_into;
use diesel::result::QueryResult;
use diesel::prelude::*;

use crate::schema;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Program {
    pub id: i64,
    pub publisher_id: Option<i64>,
    pub name: String,
    pub version: Option<String>
}

impl Program {
    pub fn upsert(
        conn: &PgConnection,
        publisher: Option<i64>,
        program_name: &String,
        v: Option<&String>
    ) -> QueryResult<i64> {
        use schema::programs::dsl::*;

        insert_into(programs)
            .values((
                publisher_id.eq(publisher),
                name.eq(program_name),
                version.eq(v)
            ))
            .on_conflict((name, version))
            .do_update()
            .set( publisher_id.eq(publisher) )
            .returning(id)
            .get_result(conn)
    }
}
