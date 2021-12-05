use crate::error::Error;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    pub datetime: NaiveDateTime,
    // うるう秒による累積のずれ (TAI - UTC)
    pub diff_seconds: i64,
}

impl LeapUtc {
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<LeapUtc, Error> {
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
    ) -> Result<Vec<LeapUtc>, Error> {
        lines
            .into_iter()
            .map(|line| LeapUtc::from_line(line.as_ref(), " ", fmt))
            .collect::<Result<Vec<_>, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};
    use rstest::*;

    #[rstest]
    #[case(
        "2017-01-02T11:22:33 15",
        " ",
        "%Y-%m-%dT%H:%M:%S",
        NaiveDate::from_ymd(2017, 1, 2).and_hms(11, 22, 33),
        15
    )]
    #[case(
        "20170102112233,15",
        ",",
        "%Y%m%d%H%M%S",
        NaiveDate::from_ymd(2017, 1, 2).and_hms(11, 22, 33),
        15
    )]
    fn test_leaps_utc_from_line(
        #[case] line: &str,
        #[case] sep: &str,
        #[case] fmt: &str,
        #[case] expected_dt: NaiveDateTime,
        #[case] expected_diff: i64,
    ) {
        let result = LeapUtc::from_line(line, sep, fmt).unwrap();

        assert_eq!(
            result,
            LeapUtc {
                datetime: expected_dt,
                diff_seconds: expected_diff
            }
        );
    }
}
