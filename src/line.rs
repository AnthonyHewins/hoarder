extern crate serde;

use serde::{de, Deserialize, Deserializer};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use chrono::NaiveDate;

fn stupid_format<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error> where D: Deserializer<'de> {
    let mut s: String = Deserialize::deserialize(deserializer)?;

    if s.len() == 0 {
        return Ok(None);
    }

    match s.find(" ") {
        Some(index) => s.replace_range(index.., ""),
        None => ()
    };
 
    match NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(de::Error::custom) {
        Ok(date) => Ok(Some(date)),
        Err(e) => Err(e)
    }
}

#[derive(Debug, Deserialize)]
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

impl Line {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Vec<Line> {
        let string = Line::read_file_to_string(path.into());
        let mut reader = csv::ReaderBuilder::new().from_reader(string.as_bytes());

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
    
    fn read_file_to_string(path: PathBuf) -> String {
        let mut vec = Vec::<u8>::new();
        File::open(path).unwrap().read_to_end(&mut vec).unwrap();
        String::from_utf8_lossy(&vec).to_string().replace("\u{FFFD}", "")
    }

    fn format_name_string(&self, old: &String) -> String {
        let trim = old.trim();
        match trim.is_empty() {
            true => trim.to_string(),
            false => trim.to_lowercase()
        }   
    }
}
