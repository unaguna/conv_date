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
    // test by month
    #[case("2021-01-26T11:06:12.123", Some(59240.0 + 39972123.0/86400000.0), None)]
    #[case("2021-02-26T11:06:12.123", Some(59271.0 + 39972123.0/86400000.0), None)]
    #[case("2021-03-26T11:06:12.123", Some(59299.0 + 39972123.0/86400000.0), None)]
    #[case("2021-04-26T11:06:12.123", Some(59330.0 + 39972123.0/86400000.0), None)]
    #[case("2021-05-26T11:06:12.123", Some(59360.0 + 39972123.0/86400000.0), None)]
    #[case("2021-06-26T11:06:12.123", Some(59391.0 + 39972123.0/86400000.0), None)]
    #[case("2021-07-26T11:06:12.123", Some(59421.0 + 39972123.0/86400000.0), None)]
    #[case("2021-08-26T11:06:12.123", Some(59452.0 + 39972123.0/86400000.0), None)]
    #[case("2021-09-26T11:06:12.123", Some(59483.0 + 39972123.0/86400000.0), None)]
    #[case("2021-10-26T11:06:12.123", Some(59513.0 + 39972123.0/86400000.0), None)]
    #[case("2021-11-26T11:06:12.123", Some(59544.0 + 39972123.0/86400000.0), None)]
    #[case("2021-12-26T11:06:12.123", Some(59574.0 + 39972123.0/86400000.0), None)]
    // test by second
    #[case("2021-12-26T00:00:00.000", Some(59574.0), None)]
    #[case("2021-12-26T00:00:00.001", Some(59574.0 + 1.0/86400000.0), None)]
    #[case("2021-12-26T00:00:01.000", Some(59574.0 + 1000.0/86400000.0), None)]
    #[case("2021-12-26T00:01:00.000", Some(59574.0 + 60000.0/86400000.0), None)]
    #[case("2021-12-26T01:00:00.000", Some(59574.0 + 3600000.0/86400000.0), None)]
    #[case("2021-12-26T12:00:00.000", Some(59574.5), None)]
    #[case("2021-12-26T13:00:00.000", Some(59574.0 + 46800000.0/86400000.0), None)]
    fn test_ut2mjd(
        #[case] utc: &str,
        #[case] expected_ok: Option<f64>,
        #[case] expected_err: Option<Error>,
    ) {
        let utc = NaiveDateTime::parse_from_str(utc, DT_FMT).unwrap();
        let expected = testmod::result(expected_ok, expected_err);

        let mjd = ut2mjd(&utc);

        match mjd {
            Err(_) => assert_eq!(mjd, expected),
            Ok(mjd) => assert_approx_eq!(mjd, expected.unwrap()),
        }
    }

    #[rstest]
    #[case(
        "2021-12-26T11:06:12.000",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("59574.46263888889"),
        None
    )]
    #[case(
        "2021-12-26T11:06:12.123",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("59574.4626403125"),
        None
    )]
    #[case(
        "2021-12-26T11:06:12",
        "%Y-%m-%dT%H:%M:%S%.3f",
        Some("59574.46263888889"),
        None
    )]
    #[case(
        "2021-12-26T11:06:12",
        "%Y-%m-%dT%H:%M:%S",
        Some("59574.46263888889"),
        None
    )]
    #[case(
        "2021-12-26 11:06:12",
        "%Y-%m-%d %H:%M:%S",
        Some("59574.46263888889"),
        None
    )]
    #[case("2021-12-26T11:06:12", "%Y-%m-%d %H:%M:%S", None, Some(Error::DatetimeParseError(utc.to_string())))]
    fn test_ut2mjd_str_arg_dt_fmt(
        #[case] utc: &str,
        #[case] dt_fmt: &str,
        #[case] expected_ok: Option<&str>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok, expected_err).map(ToString::to_string);

        let mjd = ut2mjd_str(utc, dt_fmt);

        assert_eq!(mjd, expected);
    }
}
