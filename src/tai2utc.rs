use crate::convtbl::{DiffUtcTai, TaiUtcTable, UtcTaiTable};
use crate::error::Error;
use chrono::{Duration, NaiveDateTime, Timelike};

/// Pick the diff object to use for calc utc from the datetime.
///
/// # Arguments
///
/// * `datetime` - A TAI datetime to convert to utc
/// * `utc_tai_table` - A UTC-TAI table
fn pick_dominant_diff<'a>(
    datetime: &NaiveDateTime,
    utc_tai_table: &'a UtcTaiTable,
) -> Result<&'a DiffUtcTai, Error> {
    // 線形探索
    let mut prev_diff: Option<&DiffUtcTai> = None;
    for diff_utc_tai in utc_tai_table.iter() {
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

/// Convert datetime
/// from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `tai_utc_table`.
///
/// # Arguments
/// * `datetime` - Datetime in TAI.
/// * `tai_utc_table` - The conversion table of TAI - UTC
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
///
/// let tai = convdate::tai2utc(
///     "2017-01-01T12:00:37.000",
///     &tai_utc_table,
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(tai, Ok("2017-01-01T12:00:00.000".to_string()));
/// ```
///
/// # See also
/// * [`tai2utc_dt`] - It is same as `tai2utc`, except that the argument and the result are [`NaiveDateTime`].
/// * [`tai2utc`](../tai2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tai2utc(datetime: &str, tai_utc_table: &TaiUtcTable, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let utc = tai2utc_dt(&datetime, tai_utc_table)?;
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
///
/// let utc = convdate::tai2utc_dt(
///     &NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 37),
///     &tai_utc_table);
///
/// assert_eq!(utc, Ok(NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 0)));
/// ```
///
/// # See also
/// * [`tai2utc`] - It is same as `tai2utc_dt`, except that the argument and the result are [`str`] and [`String`].
/// * [`tai2utc`](../tai2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tai2utc_dt(
    datetime: &NaiveDateTime,
    tai_utc_table: &TaiUtcTable,
) -> Result<NaiveDateTime, Error> {
    let utc_tai_table = tai_utc_table.into();
    return pick_dominant_diff(datetime, &utc_tai_table).map(|diff_utc_tai| {
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
    use crate::convtbl::DiffTaiUtc;
    use chrono::NaiveDate;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    #[rstest]
    #[case("2017-01-02T11:22:33.000", "2017-01-02T11:23:10.000")]
    #[case("2017-01-02T11:22:33.123", "2017-01-02T11:23:10.123")]
    // うるう秒が挿入される瞬間のテスト
    #[case("2016-12-31T23:59:59.000", "2017-01-01T00:00:35.000")]
    #[case("2016-12-31T23:59:60.000", "2017-01-01T00:00:36.000")]
    #[case("2016-12-31T23:59:60.123", "2017-01-01T00:00:36.123")]
    #[case("2017-01-01T00:00:00.000", "2017-01-01T00:00:37.000")]
    // うるう秒が削除される瞬間のテスト
    #[case("2017-12-31T23:59:58.000", "2018-01-01T00:00:35.000")]
    #[case("2017-12-31T23:59:58.123", "2018-01-01T00:00:35.123")]
    #[case("2018-01-01T00:00:00.000", "2018-01-01T00:00:36.000")]
    // うるう秒が2秒挿入される瞬間のテスト
    #[case("2018-12-31T23:59:59.000", "2019-01-01T00:00:35.000")]
    // #[case("2018-12-31T23:59:60.000", "2019-01-01T00:00:36.000")]
    // #[case("2018-12-31T23:59:61.000", "2019-01-01T00:00:37.000")]
    #[case("2019-01-01T00:00:00.000", "2019-01-01T00:00:38.000")]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2019-12-31T23:59:57.000", "2020-01-01T00:00:35.000")]
    #[case("2020-01-01T00:00:00.000", "2020-01-01T00:00:36.000")]
    fn test_tai2utc(#[case] expected_utc: &str, #[case] tai: &str) {
        let tai_utc_table = vec![
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
        ];
        let utc = tai2utc(&tai, &tai_utc_table.into(), DT_FMT);

        assert_eq!(utc, Ok(expected_utc.to_string()));
    }

    #[test]
    fn test_error_on_illegal_format() {
        let tai = "2019-12-31 23:59:57.000";
        let tai_utc_table = vec![DiffTaiUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = tai2utc(&tai, &tai_utc_table.into(), DT_FMT);

        assert_eq!(error, Err(Error::DatetimeParseError(tai.to_string())))
    }

    #[test]
    fn test_error_on_too_low_datetime() {
        let tai = "2015-07-01T00:00:35.999";
        let tai_utc_table = vec![
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            DiffTaiUtc {
                datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
        ];
        let error = tai2utc(&tai, &tai_utc_table.into(), DT_FMT);

        assert_eq!(
            error,
            Err(Error::DatetimeTooLowError(
                "2015-07-01 00:00:35.999".to_string()
            ))
        )
    }
}
