use rocket_contrib::json::Json;

use diesel::prelude::*;

use crate::{schema::programs, models::program::Program};
use super::wildcard;

#[get("/programs?<name>&\
       <id_start>&<id_end>&\
       <publisher_id>&\
       <version>", format = "*/*")]
pub fn index(
    conn: crate::Db,
    id_start: Option<i64>,
    id_end: Option<i64>,
    name: Option<String>,
    publisher_id: Option<i64>,
    version: Option<String>
) -> Json<Vec::<Program>> {
    let mut q = programs::table.into_boxed();

    if id_start.is_some() {
        q = q.filter(
            programs::id.ge( id_start.unwrap() )
        );
    }

    if id_end.is_some() {
        q = q.filter(
            programs::id.le( id_end.unwrap() )
        );
    }

    if name.is_some() {
        q = q.filter(
            programs::name.ilike( wildcard(&name.unwrap()) )
        );
    }

    if version.is_some() {
        q = q.filter(
            programs::version.ilike( wildcard(&version.unwrap()) )
        );
    }

    if publisher_id.is_some() {
        q = q.filter(
            programs::publisher_id.eq( publisher_id.unwrap() )
        );
    }
 
    Json(q.load::<Program>(&*conn).unwrap())
}
