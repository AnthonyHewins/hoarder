//use chrono::{ParseError, NaiveDateTime};

// CRUD endpoints
pub mod domains;
pub mod machines;
pub mod machines_programs;
pub mod programs;
pub mod publishers;

pub mod solar_winds;

pub fn wildcard<'a>(s: &'a str) -> String {
    format!("%{}%", s)
}

/*
static PARAM_DATE: &'static str = "%Y-%m-%d";

pub fn parse_date<'a>(s: &'a str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(s, PARAM_DATE)
}

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
