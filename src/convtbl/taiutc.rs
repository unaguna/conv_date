use crate::error::Error;
use chrono::NaiveDateTime;

/// Difference (TAI - UTC) and the datetime at which it is applied
///
/// It expresses a row of [the TAI-UTC table](https://www.ietf.org/timezones/data/leap-seconds.list).
///
/// # See also
/// - [`TaiUtcTable`] - It express the TAI-UTC table.
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
    /// use convdate::convtbl::DiffTaiUtc;
    /// use chrono::NaiveDate;
    ///
    /// let diff_tai_utc = DiffTaiUtc::from_line("2017-01-01T00:00:00 37", " ", "%Y-%m-%dT%H:%M:%S");
    /// assert_eq!(diff_tai_utc, Ok(DiffTaiUtc {
    ///     datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
    ///     diff_seconds: 37,
    /// }));
    /// ```
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<DiffTaiUtc, Error> {
        let parts: Vec<&str> = line.splitn(3, sep).collect();
        if parts.len() != 2 {
            Err(Error::TaiUtcTableParseError(line.to_string()))?;
        }

        let datetime = NaiveDateTime::parse_from_str(parts[0], fmt)
            .map_err(|_| Error::TaiUtcTableDatetimeParseError(parts[0].to_string()))?;

        let diff_seconds: i64 = parts[1]
            .parse()
            .map_err(|_| Error::TaiUtcTableParseError(line.to_string()))?;

        Ok(DiffTaiUtc {
            datetime,
            diff_seconds,
        })
    }
}

/// TAI-UTC conversion table
///
/// It expresses [the TAI-UTC table](https://www.ietf.org/timezones/data/leap-seconds.list); it is used for conversion from UTC to TAI.
///
/// # As Iterable Object
///
/// It behaves as an iterable object of row; for example:
///
/// ```
/// use convdate::convtbl::TaiUtcTable;
/// use chrono::NaiveDate;
///
/// let table = TaiUtcTable::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
/// for row in table.iter() {
///     assert_eq!(row.datetime, NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0));
///     assert_eq!(row.diff_seconds, 37);
/// }
/// ```
pub struct TaiUtcTable {
    diff_list: Vec<DiffTaiUtc>,
}

impl TaiUtcTable {
    /// Construct `TaiUtcTable` from lines of the TAI-UTC table file.
    ///
    /// # Arguments
    /// - `lines` - a iterable of lines of the TAI-UTC table file
    /// - `fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of datetimes in the TAI-UTC table file
    ///
    /// # Returns
    /// Returns the `TaiUtcTable` if `lines` are collect.
    ///
    /// Returns [`Error`](crate::error::Error) if some of `lines` are illegal.
    // TODO: Add sep to arguments
    pub fn from_lines(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
        fmt: &str,
    ) -> Result<TaiUtcTable, Error> {
        let diff_list: Vec<DiffTaiUtc> = lines
            .into_iter()
            .map(|line| DiffTaiUtc::from_line(line.as_ref(), " ", fmt))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(TaiUtcTable { diff_list })
    }

    /// Pick the row to use to calculate TAI from the UTC datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - An UTC datetime to convert to TAI
    pub fn pick_dominant_row<'a>(
        &'a self,
        datetime: &NaiveDateTime,
    ) -> Result<&'a DiffTaiUtc, Error> {
        // 線形探索
        let mut prev_diff: Option<&DiffTaiUtc> = None;
        for diff_utc_tai in self.iter() {
            if datetime < &diff_utc_tai.datetime {
                break;
            }
            prev_diff = Some(diff_utc_tai);
        }
        return match prev_diff {
            Some(diff_utc_tai) => Ok(diff_utc_tai),
            None => Err(Error::DatetimeTooLowError(datetime.to_string()))?,
        };
    }
}

impl From<Vec<DiffTaiUtc>> for TaiUtcTable {
    fn from(diff_list: Vec<DiffTaiUtc>) -> Self {
        TaiUtcTable { diff_list }
    }
}

impl std::ops::Deref for TaiUtcTable {
    type Target = [DiffTaiUtc];
    fn deref(&self) -> &[DiffTaiUtc] {
        self.diff_list.deref()
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
    fn test_diff_tai_utc_from_line(
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
    fn test_diff_tai_utc_from_illegal_line() {
        let line = "2017-01-02T11:22:33 15 1";
        let result = DiffTaiUtc::from_line(line, " ", "%Y-%m-%dT%H:%M:%S");

        assert_eq!(result, Err(Error::TaiUtcTableParseError(line.to_string())))
    }

    #[test]
    fn test_diff_tai_utc_from_illegal_datetime() {
        let line = "2017-01-0211:22:33 15";
        let result = DiffTaiUtc::from_line(line, " ", "%Y-%m-%dT%H:%M:%S");

        assert_eq!(
            result,
            Err(Error::TaiUtcTableDatetimeParseError(
                "2017-01-0211:22:33".to_string()
            ))
        )
    }

    #[rstest]
    #[case(
        NaiveDate::from_ymd(2012, 6, 30).and_hms_milli(23, 59, 59, 1_000),
        None,
        Some(Error::DatetimeTooLowError("2012-06-30 23:59:60".to_string())),
    )]
    #[case(
        NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 0),
        Some(DiffTaiUtc{datetime: NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 0), diff_seconds: 35}),
        None,
    )]
    #[case(
        NaiveDate::from_ymd(2015, 6, 30).and_hms_milli(23, 59, 59, 1_000),
        Some(DiffTaiUtc{datetime: NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 0), diff_seconds: 35}),
        None,
    )]
    #[case(
        NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
        Some(DiffTaiUtc{datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0), diff_seconds: 36}),
        None,
    )]
    fn test_pick_dominant_row<'a>(
        #[case] dt_input: NaiveDateTime,
        #[case] expected_ok: Option<DiffTaiUtc>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = expected_ok.as_ref().ok_or_else(|| expected_err.unwrap());

        let tai_utc_table = TaiUtcTable::from_lines(
            vec!["20120701000000 35", "20150701000000 36"],
            "%Y%m%d%H%M%S",
        )
        .unwrap();

        let dominant_row = tai_utc_table.pick_dominant_row(&dt_input);

        assert_eq!(dominant_row, expected);
    }
}
