use rocket::Data;
use rocket_contrib::json::{JsonValue, Json};
use chrono::NaiveDate;

use crate::sw::{error::SwError, upsert_bytes};

#[post("/sw_upload?<generation_date>", data = "<file>")]
pub fn upload(conn: crate::Db, generation_date: String, file: Data) -> Json<JsonValue> {
    super::json_response(move || {
        let date = parse_date(generation_date)?;
        
        let mut buf = super::read_file(file).map_err(|e| {
            SwError::UnexpectedError {
                err: format!(
                    "error with request body: {}. This may be temporary, try again",
                    e.to_string()
                )
            }
        })?;

        upsert_bytes(&*conn, &mut buf, date)
    })
}

#[get("/sw_upload/diff?<start>&<end>")]
pub fn diff(conn: crate::Db, start: String, end: String) -> Json<JsonValue> {
    use crate::models::sw_report::SwReport;

    // The reason I had to make this is because SwReport::diff
    // returns DiffError, but parse_date returns SwError, and we need to return
    // immediately if the parsing fails. But the type difference doesn't allow the ? operator
    // (because there could be two error types) so I needed this macro
    macro_rules! return_different_error_on_Err {
        ( $date:expr ) => {
            match parse_date($date) {
                Ok(d) => d,
                Err(e) => return super::json_err(e)
            };
        }
    }

    let d1 = return_different_error_on_Err!(start);
    let d2 = return_different_error_on_Err!(end);

    super::json_response(move || SwReport::diff(&conn, d1, d2))
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
