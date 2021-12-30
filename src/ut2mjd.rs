use crate::error::Error;
use chrono::{Datelike, NaiveDateTime, Timelike};

pub fn ut2mjd_str(datetime: &str, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let mjd = ut2mjd(&datetime)?;
    Ok(mjd.to_string())
}

pub fn ut2mjd(datetime: &NaiveDateTime) -> Result<f64, Error> {
    let year: i64 = datetime.year().into();
    let month_shift: i64 = ((datetime.month() + 10) % 12).into();
    let day: i64 = datetime.day().into();

    // mjd_date = [365.25 * y] + [y / 400] - [y / 100] + [30.59(m-2)] + d - 678912
    let mjd_date: i64 =
        (1461 * year / 4) + (year / 400) - (year / 100) + (3059 * month_shift / 100) + day - 678912;

    let nanoseconds_from_midnight: u64 = (datetime.num_seconds_from_midnight() as u64)
        * 1_000_000_000
        + datetime.nanosecond() as u64;
    let mjd_time: f64 = nanoseconds_from_midnight as f64 / 86_400_000_000_000.0;

    let mjd: f64 = mjd_date as f64 + mjd_time;

    return Ok(mjd);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testmod;
    use assert_approx_eq::*;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    #[rstest]
    #[case("2021-12-26T11:06:12.123", Some(59574.0 + 39972123.0/86400000.0), None)]
    // Error when the input datetime is illegal format.
    #[case("2019-12-31 23:59:57.000", None, Some(Error::DatetimeParseError(utc.to_string())))]
    fn test_ut2mjd_str(
        #[case] utc: &str,
        #[case] expected_ok: Option<f64>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok, expected_err);

        // TODO: 結果を数値化せず、文字列のまま検証する。数値の検証は別途ut2mjd_dtの試験として行う。
        let mjd = ut2mjd_str(utc, DT_FMT).map(|s| s.parse::<f64>().unwrap());

        match mjd {
            Err(_) => assert_eq!(mjd, expected),
            Ok(mjd) => assert_approx_eq!(mjd, expected.unwrap()),
        }
    }

    #[rstest]
    #[case("2021-12-26T11:06:12.000", "%Y-%m-%dT%H:%M:%S%.3f", Some(59574.0 + 39972000.0/86400000.0), None)]
    #[case("2021-12-26T11:06:12.123", "%Y-%m-%dT%H:%M:%S%.3f", Some(59574.0 + 39972123.0/86400000.0), None)]
    #[case("2021-12-26T11:06:12", "%Y-%m-%dT%H:%M:%S%.3f", Some(59574.0 + 39972000.0/86400000.0), None)]
    #[case("2021-12-26T11:06:12", "%Y-%m-%dT%H:%M:%S", Some(59574.0 + 39972000.0/86400000.0), None)]
    #[case("2021-12-26 11:06:12", "%Y-%m-%d %H:%M:%S", Some(59574.0 + 39972000.0/86400000.0), None)]
    #[case("2021-12-26T11:06:12", "%Y-%m-%d %H:%M:%S", None, Some(Error::DatetimeParseError(utc.to_string())))]
    fn test_ut2mjd_str_arg_dt_fmt(
        #[case] utc: &str,
        #[case] dt_fmt: &str,
        #[case] expected_ok: Option<f64>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok, expected_err);

        // TODO: 結果を数値化せず、文字列のまま検証する。数値の検証は別途ut2mjd_dtの試験として行う。
        let mjd = ut2mjd_str(utc, dt_fmt).map(|s| s.parse::<f64>().unwrap());

        match mjd {
            Err(_) => assert_eq!(mjd, expected),
            Ok(mjd) => assert_approx_eq!(mjd, expected.unwrap()),
        }
    }
}
