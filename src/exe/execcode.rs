//! The execcode of binary crates.
//!
//! In this module, the processing code for executable files is specified.
//! The module [`super::error`] specifies errors in executables, and cooperates with this module.

use super::error::Error;

pub const EXIT_CODE_OK: i32 = 0;
pub const EXIT_CODE_NG: i32 = 1;
pub const EXIT_CODE_SOME_DT_NOT_CONVERTED: i32 = 2;

pub fn execcode(result: &Result<(), Error>) -> i32 {
    match result {
        Ok(()) => EXIT_CODE_OK,
        Err(Error::FailedSomeConvertionError()) => EXIT_CODE_SOME_DT_NOT_CONVERTED,
    }
}
