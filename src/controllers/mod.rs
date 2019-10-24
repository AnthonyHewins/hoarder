use rocket::Data;

use chrono::{ParseError, NaiveDate};

// CRUD endpoints
pub mod domains;
pub mod machines;
pub mod machines_program;
pub mod programs;
pub mod publishers;

pub mod solar_winds;

static PARAM_DATE: &'static str = "%Y%m%d";

pub fn wildcard<'a>(s: &'a str) -> String {
    format!("%{}%", s)
}

pub fn parse_date<'a>(s: &'a str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(s, PARAM_DATE)
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

/*
fn date_range_filter<'b, 'a, T>(col: T, start: Option<&'b str>, stop: Option<&'a str>) -> T  {
    match start {
        None => {
            match stop {
                None => None,
                Some(datestring) => {
                    let stop_date = parse_date(datestring);
                    if stop_date.is_ok()
                        col.le(stop_date)
                }
            }
        },
        Some => {
            match stop {
                
            }
        }
    }
}
*/
