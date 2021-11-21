mod tai2utc;
mod tt;
mod utc2tai;
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
pub use tai2utc::tai2utc;
pub use tt::tt2tai;
pub use utc2tai::utc2tai;

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    pub datetime: DateTime<Utc>,
    // うるう秒による累積のずれ (TAI - UTC)
    pub diff_seconds: i64,
}

impl LeapUtc {
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<LeapUtc, String> {
        let parts: Vec<&str> = line.splitn(3, sep).collect();
        if parts.len() != 2 {
            return Err(format!("Illegal leap definition (block size): {}", line));
        }
        let datetime = Utc.datetime_from_str(parts[0], fmt);
        let datetime = match datetime {
            Ok(datetime) => datetime,
            Err(_e) => {
                return Err(format!(
                    "Illegal leap definition (datetime format): {}",
                    line
                ))
            }
        };
        let diff_seconds: Result<i64, _> = parts[1].parse();
        let diff_seconds = match diff_seconds {
            Ok(diff_seconds) => diff_seconds,
            Err(_e) => return Err(format!("Illegal leap definition (delta seconds): {}", line)),
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
fn normalize_leap(datetime: &DateTime<Utc>) -> NaiveDateTime {
    return NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day()).and_hms(
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    ) + Duration::nanoseconds(datetime.nanosecond().into());
}
