use rocket_contrib::json::Json;

use diesel::prelude::*;

use crate::{schema::machines, models::machine::Machine};
use super::*;

#[get("/machines?\
       <id_start>&<id_end>&\
       <user>&\
       <host>&\
       <domain_id>",
      format = "*/*")]
pub fn index(
    conn: crate::Db,
    user: Option<i64>,
    host: Option<String>,
    id_start: Option<i64>,
    id_end: Option<i64>,
    domain_id: Option<i64>
) -> Json<Vec::<Machine>> {
    let mut q = machines::table.into_boxed();

    // Metadata
    if id_start.is_some() {
        q = q.filter(
            machines::id.ge( id_start.unwrap() )
        );
    }

    if id_end.is_some() {
        q = q.filter(
            machines::id.le( id_end.unwrap() )
        );
    }

    // User data
    if user.is_some() {
        q = q.filter(
            machines::employee_id.eq( user.unwrap() )
        );
    }

    if host.is_some() {
        q = q.filter(
            machines::host.ilike( wildcard(&host.unwrap()) )
        );
    }

    if domain_id.is_some() {
        q = q.filter(
            machines::domain_id.eq( domain_id.unwrap() )
        );
    }
    
    Json(q.load::<Machine>(&*conn).unwrap())
}
