use rocket_contrib::json::{JsonValue, Json};

use diesel::prelude::*;

use crate::{schema::domains, models::domain::Domain};
use super::wildcard;

#[get("/domains?<name>&<id_start>&<id_end>", format = "*/*")]
pub fn index(
    conn: crate::Db,
    name: Option<String>,
    id_start: Option<i64>,
    id_end: Option<i64>,
) -> Json<JsonValue> {
    super::json_response(|| {
        let mut q = domains::table.into_boxed();

        if id_start.is_some() {
            q = q.filter(
                domains::id.ge( id_start.unwrap() )
            );
        }

        if id_end.is_some() {
            q = q.filter(
                domains::id.le( id_end.unwrap() )
            );
        }

        if name.is_some() {
            q = q.filter(
                domains::name.ilike( wildcard(name.as_ref().unwrap()) )
            );
        }

        q.load::<Domain>(&*conn).map_err(|e| e.to_string())
    })
}
