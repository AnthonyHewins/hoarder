use rocket::Data;
use rocket_contrib::json::{JsonValue, Json};

use crate::sw::upsert_bytes;

#[post("/sw_upload?<generation_date>", data = "<file>")]
pub fn upload(conn: crate::Db, generation_date: String, file: Data) -> Json<JsonValue> {
    let date = match super::parse_date(&generation_date) {
        Ok(date) => date,
        Err(e) => return Json(json!({
            "status": "error",
            "data": format!("error with generation_date: {}", e.to_string())
        }))
    };
    let mut buf = match super::read_file(file) {
        Ok(buf) => buf,
        Err(e) => return Json(json!({
            "status": "error",
            "data": format!("error with request body: {}", e.to_string())
        }))
    };

    let errors = upsert_bytes(&*conn, &mut buf, date);
    if errors.len() > 0 {
        Json(json!({ "status": "error", "data": errors }))
    } else {
        Json(json!({ "status": "ok", "data": "k" }))
    }
}
