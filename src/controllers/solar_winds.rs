use rocket::Data;
use rocket_contrib::json::{JsonValue, Json};

use crate::sw::upload::upsert_bytes;

#[post("/sw_upload", data = "<file>")]
pub fn upload(conn: crate::Db, file: Data) -> Json<JsonValue> {
    let mut buf = vec![];
    match file.stream_to(&mut buf) {
        Ok(n) => println!("Read {} bytes", n),
        Err(e) => {
            let s = format!("Encountered a r/w error; read {} bytes", e);
            println!("{}", s);
            return Json(json!({ "status": "error", "data": s }));
        }
    }

    let errors = upsert_bytes(&mut buf, &*conn);
    if errors.len() > 0 {
        Json(json!({ "status": "error", "data": errors }))
    } else {
        Json(json!({ "status": "ok", "data": "k" }))
    }
}
