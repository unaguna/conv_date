use crate::{error::Error, normalize_leap, DiffTaiUtc};
use chrono::{Duration, NaiveDateTime};

/// Pick the diff object to use for calc tai from the datetime.
///
/// # Arguments
///
/// * `datetime` - A datetime to convert to tai
/// * `tai_utc_table` - A TAI-UTC table
fn pick_dominant_diff<'a>(
    datetime: &NaiveDateTime,
    tai_utc_table: &'a [DiffTaiUtc],
) -> Result<&'a DiffTaiUtc, Error> {
    // 線形探索
    let mut prev_diff: Option<&DiffTaiUtc> = None;
    for diff_utc_tai in tai_utc_table.iter() {
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
/// from [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time)
/// to [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time).
///
/// This function takes leap seconds into account along the argument `tai_utc_table`.
///
/// # Arguments
/// * `datetime` - Datetime in UTC.
/// * `tai_utc_table` - The conversion table of TAI - UTC
/// * `dt_fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of `datetime`
///
/// # Returns
/// Returns the datetime in TAI.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate::{self, DiffTaiUtc};
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let tai_utc_table = DiffTaiUtc::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
///
/// let tai = convdate::utc2tai(
///     "2017-01-01T12:00:00.000",
///     &tai_utc_table,
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(tai, Ok("2017-01-01T12:00:37.000".to_string()));
/// ```
///
/// # See also
/// * [`utc2tai_dt`] - It is same as `utc2tai`, except that the argument and the result are [`NaiveDateTime`].
/// * [`utc2tai`](../utc2tai/index.html) (Binary crate) - The executable program which do same conversion.
pub fn utc2tai(
    datetime: &str,
    tai_utc_table: &[DiffTaiUtc],
    dt_fmt: &str,
) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = utc2tai_dt(&datetime, tai_utc_table)?;
    Ok(tai.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time)
/// to [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time).
///
/// This function takes leap seconds into account along the argument `tai_utc_table`.
///
/// # Arguments
/// * `datetime` - Datetime in UTC.
/// * `tai_utc_table` - The conversion table of TAI - UTC
///
/// # Returns
/// Returns the datetime in TAI.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate::{self, DiffTaiUtc};
/// use chrono::NaiveDate;
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let tai_utc_table = DiffTaiUtc::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
///
/// let tai = convdate::utc2tai_dt(
///     &NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 0),
///     &tai_utc_table);
///
/// assert_eq!(tai, Ok(NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 0, 37)));
/// ```
///
/// # See also
/// * [`utc2tai`] - It is same as `utc2tai_dt`, except that the argument and the result are [`str`] and [`String`].
/// * [`utc2tai`](../utc2tai/index.html) (Binary crate) - The executable program which do same conversion.
pub fn utc2tai_dt(
    datetime: &NaiveDateTime,
    tai_utc_table: &[DiffTaiUtc],
) -> Result<NaiveDateTime, Error> {
    let datetime_nm = normalize_leap(datetime);

    return pick_dominant_diff(datetime, tai_utc_table)
        .map(|diff_tai_utc| datetime_nm + Duration::seconds(diff_tai_utc.diff_seconds));
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[case("2018-12-31T23:59:60.000", "2019-01-01T00:00:36.000")]
    // #[case("2018-12-31T23:59:61.000", "2019-01-01T00:00:37.000")]
    #[case("2019-01-01T00:00:00.000", "2019-01-01T00:00:38.000")]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2019-12-31T23:59:57.000", "2020-01-01T00:00:35.000")]
    #[case("2020-01-01T00:00:00.000", "2020-01-01T00:00:36.000")]
    fn test_utc2tai(#[case] utc: &str, #[case] expected_tai: &str) {
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
        let tai = utc2tai(&utc, &tai_utc_table, DT_FMT);

        assert_eq!(tai, Ok(expected_tai.to_string()));
    }

    #[test]
    fn test_error_on_illegal_format() {
        let utc = "2019-12-31 23:59:57.000";
        let tai_utc_table = vec![DiffTaiUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = utc2tai(&utc, &tai_utc_table, DT_FMT);

        assert_eq!(error, Err(Error::DatetimeParseError(utc.to_string())))
    }

    #[test]
    fn test_error_on_too_low_datetime() {
        let utc = "2015-06-30T23:59:60.999";
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
        let error = utc2tai(&utc, &tai_utc_table, DT_FMT);

        assert_eq!(
            error,
            Err(Error::DatetimeTooLowError(
                "2015-06-30 23:59:60.999".to_string()
            ))
        )
    }
}
