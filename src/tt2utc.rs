use crate::convtbl::UtcTaiTable;
use crate::error::Error;
use crate::tai2utc::tai2utc_dt;
use crate::tt::tt2tai_dt;
use chrono::NaiveDateTime;

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `utc_tai_table`.
///
/// # Arguments
/// * `datetime` - Datetime in TT.
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
/// let utc = convdate::tt2utc(
///     "2017-01-01T12:01:09.000",
///     &utc_tai_table,
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(utc, Ok("2017-01-01T11:59:59.816".to_string()));
/// ```
///
/// # See also
/// * [`tt2utc`](../tt2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tt2utc(datetime: &str, utc_tai_table: &UtcTaiTable, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = tt2utc_dt(&datetime, utc_tai_table)?;
    Ok(tai.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
///
/// This function takes leap seconds into account along the argument `utc_tai_table`.
///
/// # Arguments
/// * `datetime` - Datetime in TT.
/// * `utc_tai_table` - The conversion table of UTC - TAI
///
/// # Returns
/// Returns the datetime in UTC.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # See also
/// * [`tt2utc`] - It is same as `tt2utc_dt`, except that the argument and the result are [`str`] and [`String`].
/// * [`tt2utc`](../tt2utc/index.html) (Binary crate) - The executable program which do same conversion.
pub fn tt2utc_dt(
    datetime: &NaiveDateTime,
    utc_tai_table: &UtcTaiTable,
) -> Result<NaiveDateTime, Error> {
    let tai = tt2tai_dt(datetime);

    match tai2utc_dt(&tai, utc_tai_table) {
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
    use crate::convtbl::{DiffTaiUtc, TaiUtcTable};
    use crate::testmod;
    use chrono::NaiveDate;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    #[rstest]
    // Error when the input datetime is too low.
    #[case("2015-07-01T00:01:08.183", None, Some(Error::DatetimeTooLowError("2015-07-01 00:01:08.183".to_string())))]
    #[case("2015-07-01T00:01:08.184", Some("2015-07-01T00:00:00.000"), None)]
    // regular cases
    #[case("2017-01-02T11:23:42.184", Some("2017-01-02T11:22:33.000"), None)]
    #[case("2017-01-02T11:23:42.307", Some("2017-01-02T11:22:33.123"), None)]
    // うるう秒が挿入される瞬間のテスト
    #[case("2017-01-01T00:01:07.184", Some("2016-12-31T23:59:59.000"), None)]
    #[case("2017-01-01T00:01:08.184", Some("2016-12-31T23:59:60.000"), None)]
    #[case("2017-01-01T00:01:08.307", Some("2016-12-31T23:59:60.123"), None)]
    #[case("2017-01-01T00:01:09.184", Some("2017-01-01T00:00:00.000"), None)]
    // うるう秒が削除される瞬間のテスト
    #[case("2018-01-01T00:01:07.184", Some("2017-12-31T23:59:58.000"), None)]
    #[case("2018-01-01T00:01:07.307", Some("2017-12-31T23:59:58.123"), None)]
    #[case("2018-01-01T00:01:08.184", Some("2018-01-01T00:00:00.000"), None)]
    // うるう秒が2秒挿入される瞬間のテスト
    #[case("2019-01-01T00:01:07.184", Some("2018-12-31T23:59:59.000"), None)]
    // #[case("2019-01-01T00:01:08.184", Some("2018-12-31T23:59:60.000"), None)]
    // #[case("2019-01-01T00:01:09.184", Some("2018-12-31T23:59:61.000"), None)]
    #[case("2019-01-01T00:01:10.184", Some("2019-01-01T00:00:00.000"), None)]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2020-01-01T00:01:07.184", Some("2019-12-31T23:59:57.000"), None)]
    #[case("2020-01-01T00:01:08.184", Some("2020-01-01T00:00:00.000"), None)]
    // Error when the input datetime is illegal format.
    #[case("2019-12-31 23:59:57.000", None, Some(Error::DatetimeParseError(tt.to_string())))]
    fn test_tai2utc(
        #[case] tt: &str,
        #[case] expected_ok: Option<&str>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok.map(ToString::to_string), expected_err);

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
        let utc = tt2utc(&tt, &utc_tai_table, DT_FMT);

        assert_eq!(utc, expected);
    }

    #[rstest]
    #[case(
        "2017-01-02T11:23:42.184",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:33.000"),
        None
    )]
    #[case(
        "2017-01-02T11:23:42.307",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:33.123"),
        None
    )]
    #[case(
        "2017-01-02T11:23:42",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("2017-01-02T11:22:32.816"),
        None
    )]
    #[case(
        "2017-01-02T11:23:42",
        "%Y-%m-%dT%H:%M:%S",
        Some("2017-01-02T11:22:32"),
        None
    )]
    #[case(
        "2017-01-02 11:23:42",
        "%Y-%m-%d %H:%M:%S",
        Some("2017-01-02 11:22:32"),
        None
    )]
    #[case(
        "2017-01-02T11:23:42",
        "%Y-%m-%d %H:%M:%S",
        None,
        Some(Error::DatetimeParseError(tt.to_string()))
    )]
    fn test_tai2utc_arg_dt_fmt(
        #[case] tt: &str,
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
        let utc = tt2utc(&tt, &utc_tai_table, dt_fmt);

        assert_eq!(utc, expected);
    }
}
