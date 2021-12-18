use crate::convtbl::UtcTaiTable;
use crate::error::Error;
use chrono::{Duration, NaiveDateTime, Timelike};

/// Convert datetime
/// from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `tai_utc_table`.
///
/// # Arguments
/// * `datetime` - Datetime in TAI.
/// * `utc_tai_table` - The conversion table of UTC - TAI
/// * `dt_fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of `datetime`
///
/// # Returns
/// Returns the datetime in UTC.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate;
/// use convdate::convtbl::TaiUtcTable;
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let tai_utc_table = TaiUtcTable::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
/// let utc_tai_table = From::from(&tai_utc_table);
///
/// let tai = convdate::tai2utc(
///     "2017-01-01T12:00:37.000",
///     &utc_tai_table,
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(tai, Ok("2017-01-01T12:00:00.000".to_string()));
/// ```
///
/// # See also
/// * [`tai2utc_dt`] - It is same as `tai2utc`, except that the argument and the result are [`NaiveDateTime`].
/// * [`tai2utc`](../tai2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tai2utc(datetime: &str, utc_tai_table: &UtcTaiTable, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let utc = tai2utc_dt(&datetime, utc_tai_table)?;
    Ok(utc.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `tai_utc_table`.
///
/// # Arguments
/// * `datetime` - Datetime in TAI.
/// * `tai_utc_table` - The conversion table of TAI - UTC
///
/// # Returns
/// Returns the datetime in UTC.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate;
/// use convdate::convtbl::TaiUtcTable;
/// use chrono::NaiveDate;
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let tai_utc_table = TaiUtcTable::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
/// let utc_tai_table = From::from(&tai_utc_table);
///
/// let utc = convdate::tai2utc_dt(
///     &NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 37),
///     &utc_tai_table);
///
/// assert_eq!(utc, Ok(NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 0)));
/// ```
///
/// # See also
/// * [`tai2utc`] - It is same as `tai2utc_dt`, except that the argument and the result are [`str`] and [`String`].
/// * [`tai2utc`](../tai2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tai2utc_dt(
    datetime: &NaiveDateTime,
    utc_tai_table: &UtcTaiTable,
) -> Result<NaiveDateTime, Error> {
    return utc_tai_table
        .pick_dominant_row(datetime)
        .map(|diff_utc_tai| {
            let mut datetime_tmp = datetime.clone();
            datetime_tmp += Duration::seconds(diff_utc_tai.diff_seconds);
            NaiveDateTime::from_timestamp(
                datetime_tmp.timestamp(),
                datetime_tmp.nanosecond() + diff_utc_tai.corr_seconds * 1_000_000_000,
            )
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convtbl::{DiffTaiUtc, TaiUtcTable};
    use chrono::NaiveDate;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    #[rstest]
    // Error when the input datetime is too low.
    #[case("2015-07-01T00:00:35.999", None, Some(Error::DatetimeTooLowError("2015-07-01 00:00:35.999".to_string())))]
    #[case("2015-07-01T00:00:36.000", Some("2015-07-01T00:00:00.000"), None)]
    // regular cases
    #[case("2017-01-02T11:23:10.000", Some("2017-01-02T11:22:33.000"), None)]
    #[case("2017-01-02T11:23:10.123", Some("2017-01-02T11:22:33.123"), None)]
    // うるう秒が挿入される瞬間のテスト
    #[case("2017-01-01T00:00:35.000", Some("2016-12-31T23:59:59.000"), None)]
    #[case("2017-01-01T00:00:36.000", Some("2016-12-31T23:59:60.000"), None)]
    #[case("2017-01-01T00:00:36.123", Some("2016-12-31T23:59:60.123"), None)]
    #[case("2017-01-01T00:00:37.000", Some("2017-01-01T00:00:00.000"), None)]
    // うるう秒が削除される瞬間のテスト
    #[case("2018-01-01T00:00:35.000", Some("2017-12-31T23:59:58.000"), None)]
    #[case("2018-01-01T00:00:35.123", Some("2017-12-31T23:59:58.123"), None)]
    #[case("2018-01-01T00:00:36.000", Some("2018-01-01T00:00:00.000"), None)]
    // うるう秒が2秒挿入される瞬間のテスト
    #[case("2019-01-01T00:00:35.000", Some("2018-12-31T23:59:59.000"), None)]
    // #[case("2019-01-01T00:00:36.000", Some("2018-12-31T23:59:60.000"), None)]
    // #[case("2019-01-01T00:00:37.000", Some("2018-12-31T23:59:61.000"), None)]
    #[case("2019-01-01T00:00:38.000", Some("2019-01-01T00:00:00.000"), None)]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2020-01-01T00:00:35.000", Some("2019-12-31T23:59:57.000"), None)]
    #[case("2020-01-01T00:00:36.000", Some("2020-01-01T00:00:00.000"), None)]
    // Error when the input datetime is illegal format.
    #[case("2019-12-31 23:59:57.000", None, Some(Error::DatetimeParseError(tai.to_string())))]
    fn test_tai2utc(
        #[case] tai: &str,
        #[case] expected_ok: Option<&str>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = expected_ok
            .map(ToString::to_string)
            .ok_or_else(|| expected_err.unwrap());

        let tai_utc_table: TaiUtcTable = vec![
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2019, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 38,
            },
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
        ]
        .into();
        let utc_tai_table = From::from(&tai_utc_table);
        let utc = tai2utc(&tai, &utc_tai_table, DT_FMT);

        assert_eq!(utc, expected);
    }

    #[rstest]
    #[case(
        "2017-01-02T11:23:10.000",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:33.000"),
        None
    )]
    #[case(
        "2017-01-02T11:23:10.123",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:33.123"),
        None
    )]
    #[case(
        "2017-01-02T11:23:10",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:33.000"),
        None
    )]
    #[case(
        "2017-01-02T11:23:10",
        "%Y-%m-%dT%H:%M:%S",
        Some("2017-01-02T11:22:33"),
        None
    )]
    #[case(
        "2017-01-02 11:23:10",
        "%Y-%m-%d %H:%M:%S",
        Some("2017-01-02 11:22:33"),
        None
    )]
    #[case(
        "2017-01-02T11:23:10",
        "%Y-%m-%d %H:%M:%S",
        None,
        Some(Error::DatetimeParseError(tai.to_string()))
    )]
    fn test_tai2utc_arg_dt_fmt(
        #[case] tai: &str,
        #[case] dt_fmt: &str,
        #[case] expected_ok: Option<&str>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = expected_ok
            .map(ToString::to_string)
            .ok_or_else(|| expected_err.unwrap());

        let tai_utc_table: TaiUtcTable = vec![DiffTaiUtc {
            datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
            diff_seconds: 37,
        }]
        .into();
        let utc_tai_table = From::from(&tai_utc_table);
        let utc = tai2utc(&tai, &utc_tai_table, dt_fmt);

        assert_eq!(utc, expected);
    }
}
