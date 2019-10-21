use std::string::ToString;

use super::line::Line;

#[derive(Debug, Fail, Serialize)]
pub enum SwError {
    #[fail(display = "error on line {}", lineno)]
    ArgError {
        lineno: usize,
        col: String,
        line: Line,
        err: String
    },
}
