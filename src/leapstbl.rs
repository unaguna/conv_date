use crate::error::Error;
use anyhow::Result;
use chrono::NaiveDateTime;

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

    pub fn from_lines(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
        fmt: &str,
    ) -> Result<Vec<LeapUtc>> {
        lines
            .into_iter()
            .map(|line| LeapUtc::from_line(line.as_ref(), " ", fmt))
            .collect::<Result<Vec<_>>>()
    }
}
