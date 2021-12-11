use crate::{error::Error, LeapUtc};
use crate::{tai2utc_dt, tt2tai_dt};
use chrono::NaiveDateTime;

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `leaps`.
///
/// # Arguments
/// * `datetime` - Datetime in TT.
/// * `leaps` - The conversion table of TAI - UTC
/// * `dt_fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of `datetime`
///
/// # Returns
/// Returns the datetime in UTC.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate::{self, LeapUtc};
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let leaps = LeapUtc::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
///
/// let utc = convdate::tt2utc(
///     "2017-01-01T12:01:09.000",
///     &leaps,
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(utc, Ok("2017-01-01T11:59:59.816".to_string()));
/// ```
///
/// # See also
/// * [`tt2utc_dt`] - It is same as `tt2utc`, except that the argument and the result are [`NaiveDateTime`].
/// * [`tt2utc`](../tt2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tt2utc(datetime: &str, leaps: &[LeapUtc], dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = tt2utc_dt(&datetime, leaps)?;
    Ok(tai.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `leaps`.
///
/// # Arguments
/// * `datetime` - Datetime in TT.
/// * `leaps` - The conversion table of TAI - UTC
///
/// # Returns
/// Returns the datetime in UTC.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate::{self, LeapUtc};
/// use chrono::NaiveDate;
///
/// // Usually, lines read from the file are used as the argument of `from_lines`.
/// let leaps = LeapUtc::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
///
/// let utc = convdate::tt2utc_dt(
///     &NaiveDate::from_ymd(2017, 1, 1).and_hms(12, 1, 9),
///     &leaps);
///
/// assert_eq!(utc, Ok(NaiveDate::from_ymd(2017, 1, 1).and_hms_milli(11, 59, 59, 816)));
/// ```
///
/// # See also
/// * [`tt2utc`] - It is same as `tt2utc_dt`, except that the argument and the result are [`str`] and [`String`].
/// * [`tt2utc`](../tt2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tt2utc_dt(datetime: &NaiveDateTime, leaps: &[LeapUtc]) -> Result<NaiveDateTime, Error> {
    let tai = tt2tai_dt(datetime);

    match tai2utc_dt(&tai, leaps) {
        Err(Error::DatetimeTooLowError(_)) => {
            // 多段階で変換を行う場合、中間の日時文字列がエラーメッセージに使われている場合があるため、入力された日時文字列に置き換える。
            Err(Error::DatetimeTooLowError(datetime.to_string()))
        }
        Err(e) => Err(e),
        Ok(utc) => Ok(utc),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    #[rstest]
    #[case("2017-01-02T11:22:33.000", "2017-01-02T11:23:42.184")]
    #[case("2017-01-02T11:22:33.123", "2017-01-02T11:23:42.307")]
    // うるう秒が挿入される瞬間のテスト
    #[case("2016-12-31T23:59:59.000", "2017-01-01T00:01:07.184")]
    #[case("2016-12-31T23:59:60.000", "2017-01-01T00:01:08.184")]
    #[case("2016-12-31T23:59:60.123", "2017-01-01T00:01:08.307")]
    #[case("2017-01-01T00:00:00.000", "2017-01-01T00:01:09.184")]
    // うるう秒が削除される瞬間のテスト
    #[case("2017-12-31T23:59:58.000", "2018-01-01T00:01:07.184")]
    #[case("2017-12-31T23:59:58.123", "2018-01-01T00:01:07.307")]
    #[case("2018-01-01T00:00:00.000", "2018-01-01T00:01:08.184")]
    // うるう秒が2秒挿入される瞬間のテスト
    #[case("2018-12-31T23:59:59.000", "2019-01-01T00:01:07.184")]
    // #[case("2018-12-31T23:59:60.000", "2019-01-01T00:01:08.184")]
    // #[case("2018-12-31T23:59:61.000", "2019-01-01T00:01:09.184")]
    #[case("2019-01-01T00:00:00.000", "2019-01-01T00:01:10.184")]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2019-12-31T23:59:57.000", "2020-01-01T00:01:07.184")]
    #[case("2020-01-01T00:00:00.000", "2020-01-01T00:01:08.184")]
    fn test_tai2utc(#[case] expected_utc: &str, #[case] tt: &str) {
        let leaps = vec![
            LeapUtc {
                datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
            LeapUtc {
                datetime: NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: NaiveDate::from_ymd(2019, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 38,
            },
            LeapUtc {
                datetime: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
        ];
        let utc = tt2utc(&tt, &leaps, DT_FMT);

        assert_eq!(utc, Ok(expected_utc.to_string()));
    }

    #[test]
    fn test_error_on_illegal_format() {
        let tt = "2019-12-31 23:59:57.000";
        let leaps = vec![LeapUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = tt2utc(&tt, &leaps, DT_FMT);

        assert_eq!(error, Err(Error::DatetimeParseError(tt.to_string())))
    }

    #[test]
    fn test_error_on_too_low_datetime() {
        let tt = "2015-07-01T00:01:08.183";
        let leaps = vec![
            LeapUtc {
                datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
        ];
        let error = tt2utc(&tt, &leaps, DT_FMT);

        assert_eq!(
            error,
            Err(Error::DatetimeTooLowError(
                "2015-07-01 00:01:08.183".to_string()
            ))
        )
    }
}
