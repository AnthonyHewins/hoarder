use rocket::Data;
use rocket_contrib::json::{JsonValue, Json};
use chrono::NaiveDate;

use crate::sw::{error::SwError, upsert_bytes};

#[post("/sw_upload?<generation_date>", data = "<file>")]
pub fn upload(conn: crate::Db, generation_date: String, file: Data) -> Json<JsonValue> {
    super::json_response(|| {
        let date = parse_date(generation_date)?;
        
        let mut buf = super::read_file(file).map_err(|e| {
            format!("error with request body: {}", e.to_string())
        })?;

        upsert_bytes(&*conn, &mut buf, date)
    })
}

#[get("/sw_upload/diff?<start>&<end>")]
pub fn diff(conn: crate::Db, start: String, end: String) -> Json<JsonValue> {
    use crate::models::sw_report::{error::DiffError, SwReport};

    super::json_response(|| {
        SwReport::diff(&conn, parse_date(start)?, parse_date(end)?)
    })
}

fn parse_date(d: String) -> Result<NaiveDate, SwError> {
    match d.parse::<NaiveDate>() {
        Ok(d) => Ok(d),
        Err(e) => Err(
            SwError::MetadataError {
                err: format!(
                    "got error '{}'; is your date param {} not of the form %Y-%m-%d?",
                    e.to_string(),
                    d.to_string()
                )
            }
        )
    }
}
