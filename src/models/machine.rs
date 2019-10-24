use diesel::PgConnection;
use diesel::insert_into;
use diesel::result::QueryResult;
use diesel::prelude::*;

use crate::schema;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Machine {
    pub id: i64,
    pub domain_id: Option<i64>,
    pub employee_id: Option<i64>,
    pub host: String,
}

impl Machine {
    pub fn upsert(conn: &PgConnection, domain: i64, employee: Option<i64>, name: &String) -> QueryResult<i64> {
        use schema::machines::dsl::*;

        insert_into(machines)
            .values(( host.eq(name), employee_id.eq(employee), domain_id.eq(domain) ))
            .on_conflict(host)
            .do_update()
            .set( domain_id.eq(domain) )
            .returning(id)
            .get_result(conn)
    }
}
