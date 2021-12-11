use std::path::PathBuf;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Illegal leap definition: {0}")]
    TaiUtcTableParseError(String),
    #[error("Illegal leap definition (datetime): {0}")]
    TaiUtcTableDatetimeParseError(String),
    #[error("The datetime is too low: {0}")]
    DatetimeTooLowError(String),
    #[error("Cannot parse the datetime: {0}")]
    DatetimeParseError(String),
    #[error("The TAI-UTC table file isn't available: {0}")]
    LeapsTableIOError(PathBuf),
    #[error("Cannot read the TAI-UTC table file as text: {0}")]
    LeapsTableNotTextError(PathBuf),
}
