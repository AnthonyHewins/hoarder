use super::line::Line;

/// This error enum is for parsing logs and serialization. The delta checker
/// separate from this file.
#[derive(Debug, Fail, Serialize)]
pub enum SwError {
    #[fail(display = "error in the upload metadata: {}", err)]
    MetadataError { err: String },

    #[fail(display = "line {}, col {}: {} (data: {})", lineno, col, err, line)]
    LineError {
        lineno: usize,
        col: String,
        line: Line,
        err: String
    },

    #[fail(display = "line {}, column {}: value is whitespace", lineno, col)]
    BlankError {
        lineno: usize,
        col: String,
    },

    #[fail(display = "line {}: got error {}", lineno, err)]
    CsvError {
        lineno: usize,
        err: String
    },

    #[fail(display = "Unexpected error: {}", err)]
    UnexpectedError { err: String }
}
