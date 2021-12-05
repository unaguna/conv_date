use crate::{error::Error, LeapUtc};
use crate::{tai2tt_dt, utc2tai_dt};
use chrono::NaiveDateTime;

pub fn utc2tt(datetime: &str, leaps: &[LeapUtc], dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = utc2tt_dt(&datetime, leaps)?;
    Ok(tai.format(dt_fmt).to_string())
}

pub fn utc2tt_dt(datetime: &NaiveDateTime, leaps: &[LeapUtc]) -> Result<NaiveDateTime, Error> {
    let tai = utc2tai_dt(datetime, leaps)?;
    Ok(tai2tt_dt(&tai))
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
    #[case("2018-12-31T23:59:60.000", "2019-01-01T00:01:08.184")]
    // #[case("2018-12-31T23:59:61.000", "2019-01-01T00:01:09.184")]
    #[case("2019-01-01T00:00:00.000", "2019-01-01T00:01:10.184")]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2019-12-31T23:59:57.000", "2020-01-01T00:01:07.184")]
    #[case("2020-01-01T00:00:00.000", "2020-01-01T00:01:08.184")]
    fn test_utc2tai(#[case] utc: &str, #[case] expected_tt: &str) {
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
        let tai = utc2tt(&utc, &leaps, DT_FMT);

        assert_eq!(tai, Ok(expected_tt.to_string()));
    }

    #[test]
    fn test_error_on_illegal_format() {
        let utc = "2019-12-31 23:59:57.000";
        let leaps = vec![LeapUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = utc2tt(&utc, &leaps, DT_FMT);

        assert_eq!(error, Err(Error::DatetimeParseError(utc.to_string())))
    }

    #[test]
    fn test_error_on_too_low_datetime() {
        let utc = "2015-06-30T23:59:60.999";
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
        let error = utc2tt(&utc, &leaps, DT_FMT);

        assert_eq!(
            error,
            Err(Error::DatetimeTooLowError(
                "2015-06-30 23:59:60.999".to_string()
            ))
        )
    }
}
