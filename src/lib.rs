pub mod error;
pub mod exe;
mod leapstbl;
mod tai2utc;
mod tt;
mod tt2utc;
mod utc2tai;
mod utc2tt;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
pub use leapstbl::LeapUtc;
pub use tai2utc::{tai2utc, tai2utc_dt};
pub use tt::{tai2tt, tai2tt_dt, tt2tai, tt2tai_dt};
pub use tt2utc::{tt2utc, tt2utc_dt};
pub use utc2tai::{utc2tai, utc2tai_dt};
pub use utc2tt::{utc2tt, utc2tt_dt};

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

/// Convert datetime to naive without leap
///
/// Nanoseconds that exceed 1000000 to represent leap seconds are added to seconds.
fn normalize_leap(datetime: &NaiveDateTime) -> NaiveDateTime {
    return NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day()).and_hms(
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    ) + Duration::nanoseconds(datetime.nanosecond().into());
}
