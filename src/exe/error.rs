//! Errors of binary crates.
//!
//! In this module, the errors of executables is specified.
//! The module [`super::execcode`] specifies the execcode of executables, and cooperates with this module.

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Failed convertion of some datetimes.")]
    FailedSomeConvertionError(),
}
