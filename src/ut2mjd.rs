use crate::error::Error;

pub fn ut2mjd(datetime: &str, dt_fmt: &str) -> Result<f64, Error> {
    panic!("Not implemented.");
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
    fn test_ut2mjd(
        #[case] utc: &str,
        #[case] expected_ok: Option<f64>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok, expected_err);
        let mjd = ut2mjd(utc, DT_FMT);

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
    fn test_utc2tai_arg_dt_fmt(
        #[case] utc: &str,
        #[case] dt_fmt: &str,
        #[case] expected_ok: Option<f64>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok, expected_err);

        let mjd = ut2mjd(utc, dt_fmt);

        match mjd {
            Err(_) => assert_eq!(mjd, expected),
            Ok(mjd) => assert_approx_eq!(mjd, expected.unwrap()),
        }
    }
}
