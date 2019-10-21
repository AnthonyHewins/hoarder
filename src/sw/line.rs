use std::fmt;
use std::path::PathBuf;
use std::str;

use serde::{de, Deserialize, Deserializer};
use chrono::NaiveDate;

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
    #[serde(rename = "Domain or Workgroup")]
    pub domain: String,

    #[serde(rename = "Computer Name")]
    pub hostname: String,

    #[serde(rename = "Display Name")]
    pub program: String,

    #[serde(rename = "Version (as string)")]
    pub version: String,

    #[serde(rename = "Publisher (Programs and Features)")]
    pub publisher: String,

    #[serde(rename = "Registered Owner")]
    pub owner: String,

    #[serde(rename = "Install Date", deserialize_with = "stupid_format")]
    pub date_installed: Option<NaiveDate>,

    #[serde(rename = "Install Location")]
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
             version: {}, \
             publisher: {}, \
             owner: {}, \
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
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Vec<Line> {
        let utf8_string = Line::delete_non_utf8_bytes(bytes);
        let mut reader = csv::ReaderBuilder::new().from_reader(
            utf8_string.as_bytes()
        );

        let mut vec = Vec::<Line>::new();
        for i in reader.deserialize() {
            let mut line: Line = i.unwrap();
            line.validate_struct_fields();
            vec.push(line);
        }
        vec
    }
    
    pub fn validate_struct_fields(&mut self) {
        self.domain = self.format_name_string(&self.domain);
        self.hostname = self.format_name_string(&self.hostname);
        self.owner = self.format_name_string(&self.owner);

        self.publisher = self.format_name_string(&self.publisher);
        self.program = self.format_name_string(&self.program);
        self.version = self.format_name_string(&self.version);
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

    fn format_name_string(&self, old: &String) -> String {
        let trim = old.trim();
        match trim.is_empty() {
            true => trim.to_string(),
            false => trim.to_lowercase()
        }   
    }
}
