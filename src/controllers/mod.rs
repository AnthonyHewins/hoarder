use rocket::Data;

use serde::Serialize;
use rocket_contrib::json::{JsonValue, Json};
use chrono::{ParseError, NaiveDate};

// CRUD endpoints
pub mod domains;
pub mod machines;
pub mod machines_program;
pub mod programs;
pub mod publishers;

pub mod solar_winds;

pub fn json_response<F, O, E>(f: F) -> Json<JsonValue>
where O: Serialize, E: Serialize, F: Fn() -> Result<O, E> {
    match f() {
        Ok(result) => Json(json!({"status": "ok", "data": result})),
        Err(e) => Json(json!({"status": "err", "data": e}))
    }
}

pub fn wildcard<'a>(s: &'a str) -> String {
    format!("%{}%", s)
}

pub fn read_file(file: Data) -> Result<Vec::<u8>, std::io::Error> {
    let mut buf = vec![];
    match file.stream_to(&mut buf) {
        Ok(n) => {
            println!("Read {} bytes", n);
            Ok(buf)
        },
        Err(e) => {
            let s = format!("Encountered a r/w error; read {} bytes", e);
            println!("{}", s);
            Err(e)
        }
    }
}
