#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Illegal leap definition: {0}")]
    LeapTableParseError(String),
    #[error("Illegal leap definition (datetime): {0}")]
    LeapTableDatetimeParseError(String),
    #[error("The datetime is too low: {0}")]
    DatetimeTooLowError(String),
}
