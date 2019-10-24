use std::fmt;
use std::path::PathBuf;
use std::str;

use serde::{de, Deserialize, Deserializer};
use chrono::NaiveDate;

use super::error::SwError;

fn stupid_format<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error> where D: Deserializer<'de> {
    let mut s: String = Deserialize::deserialize(deserializer)?;

    if s.len() == 0 {
        return Ok(None);
    }

    // This isn't correct, this should remove all whitespace (\s)
    match s.find(" ") {
        Some(index) => s.replace_range(index.., ""),
        None => ()
    };

    match NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(de::Error::custom) {
        Ok(date) => Ok(Some(date)),
        Err(e) => Err(e)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Line {
    #[serde(rename = "domain or workgroup")]
    pub domain: String,

    #[serde(rename = "computer name")]
    pub hostname: String,

    #[serde(rename = "display name")]
    pub program: String,

    #[serde(rename = "version (as string)")]
    pub version: Option<String>,

    #[serde(rename = "publisher (programs and features)")]
    pub publisher: Option<String>,

    #[serde(rename = "registered owner")]
    pub owner: Option<String>,

    #[serde(rename = "install date", deserialize_with = "stupid_format")]
    pub date_installed: Option<NaiveDate>,

    #[serde(rename = "install location")]
    pub path: Option<PathBuf>
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\
             domain: {}, \
             hostname: {}, \
             program: {}, \
             version: {:?}, \
             publisher: {:?}, \
             owner: {:?}, \
             date_installed: {:?}, \
             path: {:?}\
             }}",
            self.domain, 
            self.hostname,
            self.program,
            self.version,
            self.publisher,
            self.owner,
            self.date_installed,
            self.path
        )
    }
}

impl Line {
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Vec<Result<Line, SwError>> {
        let utf8_string = Line::delete_non_utf8_bytes(bytes);
        let mut reader = csv::ReaderBuilder::new().from_reader(
            utf8_string.as_bytes()
        );

        let mut lineno = 1;
        let mut vec = Vec::<Result<Line, SwError>>::new();
        for i in reader.deserialize::<Line>() {
            match i {
                Ok(mut line) => {
                    match line.validate_struct_fields(lineno) {
                        Ok(()) => vec.push(Ok(line)),
                        Err(e) => vec.push(Err(e))
                    }
                },
                Err(e) => {
                    vec.push( Err(SwError::CsvError {
                        lineno: lineno,
                        err: e.to_string()
                    }));
                }
            }
            lineno += 1;
        }
        vec
    }

    fn delete_non_utf8_bytes<'a>(bytes: &'a [u8]) -> String {
        let mut s = String::new();
        let mut buf: &[u8] = bytes;

        loop {
            match str::from_utf8(buf) {
                Ok(utf8_string) => {
                    s.push_str(utf8_string);
                    return s;
                },
                Err(e) => {
                    let (valid_utf8, invalid_utf8) = buf.split_at(e.valid_up_to());

                    if !valid_utf8.is_empty() {
                        let known_valid_part = unsafe {
                            str::from_utf8_unchecked(valid_utf8)
                        };
                        s.push_str(known_valid_part);
                    }

                    if invalid_utf8.is_empty() {
                        return s;
                    }

                    buf = &invalid_utf8[1..];
                }
            }
        }
    }

    pub fn validate_struct_fields(&mut self, lineno: usize) -> Result<(), SwError> {
        self.domain = self.format_string(&self.domain, lineno, "domain".to_string())?;
        self.hostname = self.format_string(&self.hostname, lineno, "hostname".to_string())?;
        self.program = self.format_string(&self.program, lineno, "program".to_string())?;

        self.owner = self.format_option_string(self.owner.as_ref());
        self.publisher = self.format_option_string(self.publisher.as_ref());
        self.version = self.format_option_string(self.version.as_ref());
        Ok(())
    }

    fn format_string(&self, old: &String, lineno: usize, col: String) -> Result<String, SwError> {
        let trim = old.trim();
        match trim.is_empty() {
            true => Err(SwError::BlankError { lineno: lineno, col: col }),
            false => Ok(trim.to_lowercase())
        }   
    }

    fn format_option_string(&self, old: Option<&String>) -> Option<String> {
        match old {
            None => None,
            Some(s) => {
                let trim = s.trim();
                match trim.is_empty() {
                    true => None,
                    false => Some(trim.to_lowercase())
                }
            }
        }
    }
}
