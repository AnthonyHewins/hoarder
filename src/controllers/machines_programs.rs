use rocket_contrib::json::Json;

use diesel::prelude::*;

use crate::{schema::machines_programs, models::machines_programs::MachinesPrograms};
use super::wildcard;

#[get("/machines_programs?<path>&\
       <id_start>&<id_end>&\
       <program_id>&\
       <machine_id>", format = "*/*")]
pub fn index(
    conn: crate::Db,
    id_start: Option<i64>,
    id_end: Option<i64>,
    path: Option<String>,
    program_id: Option<i64>,
    machine_id: Option<i64>,
) -> Json<Vec::<MachinesPrograms>> {
    let mut q = machines_programs::table.into_boxed();

    if id_start.is_some() {
        q = q.filter(
            machines_programs::id.ge( id_start.unwrap() )
        );
    }

    if id_end.is_some() {
        q = q.filter(
            machines_programs::id.le( id_end.unwrap() )
        );
    }

    if path.is_some() {
        q = q.filter(
            machines_programs::path.ilike( wildcard(&path.unwrap()) )
        );
    }

    if machine_id.is_some() {
        q = q.filter(
            machines_programs::machine_id.eq( machine_id.unwrap() )
        );
    }
 
    if program_id.is_some() {
        q = q.filter(
            machines_programs::program_id.eq( program_id.unwrap() )
        );
    }
 
    Json(q.load::<MachinesPrograms>(&*conn).unwrap())
}
