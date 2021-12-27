//! convdate is a set of tools for converting datetime UTC <=> TAI <=> TT.
//!
//! If you want **to know about executables, look for below documents** of binary crates:
//! - [tai2utc](../tai2utc/index.html)
//! - [tt2utc](../tt2utc/index.html)
//! - [utc2tai](../utc2tai/index.html)
//! - [utc2tt](../utc2tt/index.html)
//!
//! This crate provide some features to above binary crates.
//!
//! # Caution
//! *This library crate is being adjusted. There are plans to make disruptive changes in future updates.*

pub mod convtbl;
pub mod error;
#[doc(hidden)]
pub mod exe;
mod tai2utc;
mod tt;
mod tt2utc;
mod ut2mjd;
mod utc2tai;
mod utc2tt;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
pub use tai2utc::tai2utc;
pub use tt::{tai2tt, tt2tai};
pub use tt2utc::tt2utc;
pub use utc2tai::utc2tai;
pub use utc2tt::utc2tt;

#[cfg(test)]
mod testmod;

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

/// Convert datetime to naive without leap
///
/// Nanoseconds that exceed 1_000_000_000 to represent leap seconds are added to seconds.
///
/// # Arguments
/// * `datetime` - Datetime which may express leap second.
///
/// # Returns
/// A datetime without leaps.
fn normalize_leap(datetime: &NaiveDateTime) -> NaiveDateTime {
    return NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day()).and_hms(
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    ) + Duration::nanoseconds(datetime.nanosecond().into());
}
