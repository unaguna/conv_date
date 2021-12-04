pub mod error;
pub mod exe;
mod tai2utc;
mod tt;
mod utc2tai;
use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
use error::Error;
pub use tai2utc::{tai2utc, tai2utc_dt};
pub use tt::{tai2tt, tai2tt_dt, tt2tai, tt2tai_dt};
pub use utc2tai::{utc2tai, utc2tai_dt};

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    pub datetime: NaiveDateTime,
    // うるう秒による累積のずれ (TAI - UTC)
    pub diff_seconds: i64,
}

impl LeapUtc {
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<LeapUtc> {
        let parts: Vec<&str> = line.splitn(3, sep).collect();
        if parts.len() != 2 {
            Err(Error::LeapTableParseError(line.to_string()))?;
        }
        let datetime = NaiveDateTime::parse_from_str(parts[0], fmt);
        let datetime = match datetime {
            Ok(datetime) => datetime,
            Err(_e) => {
                return Err(Error::LeapTableDatetimeParseError(parts[0].to_string()))?;
            }
        };
        let diff_seconds: Result<i64, _> = parts[1].parse();
        let diff_seconds = match diff_seconds {
            Ok(diff_seconds) => diff_seconds,
            Err(_e) => return Err(Error::LeapTableParseError(line.to_string()))?,
        };
        Ok(LeapUtc {
            datetime,
            diff_seconds,
        })
    }
}

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
