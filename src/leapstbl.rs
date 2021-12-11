use crate::error::Error;
use chrono::NaiveDateTime;

/// Difference (TAI - UTC) and the datetime at which it is applied
///
/// It express a row of [the TAI-UTC table](https://web.archive.org/web/20191019051734/http://maia.usno.navy.mil/ser7/tai-utc.dat).
/// So `Vec<DiffTaiUtc>` express the TAI-UTC table.
#[derive(Debug, PartialEq)]
pub struct DiffTaiUtc {
    /// (UTC) The moment when the difference (TAI - UTC) changes due to a leap second
    pub datetime: NaiveDateTime,
    /// The difference (TAI - UTC)
    pub diff_seconds: i64,
}

impl DiffTaiUtc {
    /// Construct `DiffTaiUtc` from line of the TAI-UTC table file.
    ///
    /// # Arguments
    /// - `line` - a line of the TAI-UTC table file
    /// - `sep` - the separator between a datetime and a difference value in `line`
    /// - `fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of datetimes in the TAI-UTC table file
    ///
    /// # Returns
    /// Returns the `DiffTaiUtc` if `line` is collect.
    ///
    /// Returns [`Error`](crate::error::Error) if `line` is illegal.
    ///
    /// # Examples
    /// ```
    /// use convdate::DiffTaiUtc;
    /// use chrono::NaiveDate;
    ///
    /// let leap_line = DiffTaiUtc::from_line("2017-01-01T00:00:00 37", " ", "%Y-%m-%dT%H:%M:%S");
    /// assert_eq!(leap_line, Ok(DiffTaiUtc {
    ///     datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
    ///     diff_seconds: 37,
    /// }));
    /// ```
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<DiffTaiUtc, Error> {
        let parts: Vec<&str> = line.splitn(3, sep).collect();
        if parts.len() != 2 {
            Err(Error::LeapTableParseError(line.to_string()))?;
        }

        let datetime = NaiveDateTime::parse_from_str(parts[0], fmt)
            .map_err(|_| Error::LeapTableDatetimeParseError(parts[0].to_string()))?;

        let diff_seconds: i64 = parts[1]
            .parse()
            .map_err(|_| Error::LeapTableParseError(line.to_string()))?;

        Ok(DiffTaiUtc {
            datetime,
            diff_seconds,
        })
    }

    /// Construct `Vec<DiffTaiUtc>` from lines of the TAI-UTC table file.
    ///
    /// # Arguments
    /// - `lines` - a iterable of lines of the TAI-UTC table file
    /// - `fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of datetimes in the TAI-UTC table file
    ///
    /// # Returns
    /// Returns the `Vec<DiffTaiUtc>` if `lines` are collect.
    ///
    /// Returns [`Error`](crate::error::Error) if some of `lines` are illegal.
    // TODO: Add sep to arguments
    pub fn from_lines(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
        fmt: &str,
    ) -> Result<Vec<DiffTaiUtc>, Error> {
        lines
            .into_iter()
            .map(|line| DiffTaiUtc::from_line(line.as_ref(), " ", fmt))
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
        let result = DiffTaiUtc::from_line(line, sep, fmt);

        assert_eq!(
            result,
            Ok(DiffTaiUtc {
                datetime: expected_dt,
                diff_seconds: expected_diff
            })
        );
    }

    #[test]
    fn test_leaps_utc_from_illegal_line() {
        let line = "2017-01-02T11:22:33 15 1";
        let result = DiffTaiUtc::from_line(line, " ", "%Y-%m-%dT%H:%M:%S");

        assert_eq!(result, Err(Error::LeapTableParseError(line.to_string())))
    }

    #[test]
    fn test_leaps_utc_from_illegal_datetime() {
        let line = "2017-01-0211:22:33 15";
        let result = DiffTaiUtc::from_line(line, " ", "%Y-%m-%dT%H:%M:%S");

        assert_eq!(
            result,
            Err(Error::LeapTableDatetimeParseError(
                "2017-01-0211:22:33".to_string()
            ))
        )
    }
}
