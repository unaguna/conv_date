use crate::{error::Error, normalize_leap, LeapUtc};
use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Timelike};

struct LeapTai {
    // うるう秒によってずれるタイミング (TAI)
    pub datetime: NaiveDateTime,
    // うるう秒による累積のずれ (UTC - TAI) のうち、60s=1mとして計算する部分
    pub diff_seconds: i64,
    // うるう秒による累積のずれ (UTC - TAI) のうち、分に繰り上がらない部分
    pub corr_seconds: u32,
}

/// Pick the leap object to use for calc utc from the datetime.
///
/// # Arguments
///
/// * `datetime` - A TAI datetime to convert to utc
/// * `leaps` - A list of leap objects
fn pick_dominant_leap<'a>(
    datetime: &NaiveDateTime,
    leaps: &'a [LeapTai],
) -> Result<&'a LeapTai, Error> {
    // 線形探索
    let mut prev_leap: Option<&LeapTai> = None;
    for leap in leaps.iter() {
        if datetime < &leap.datetime {
            break;
        }
        prev_leap = Some(leap);
    }
    return match prev_leap {
        Some(leap) => Ok(leap),
        None => Err(Error::DatetimeTooLowError(datetime.to_string()))?,
    };
}

fn utc_leaps_to_tai_leaps(leaps: &[LeapUtc]) -> Vec<LeapTai> {
    let mut tai_leaps = Vec::new();
    let mut prev_leap_diff = i64::MAX;
    for leap in leaps.iter() {
        if prev_leap_diff < leap.diff_seconds {
            let corr_seconds = leap.diff_seconds - prev_leap_diff;
            tai_leaps.push(LeapTai {
                datetime: normalize_leap(&leap.datetime)
                    + Duration::seconds(leap.diff_seconds - corr_seconds),
                diff_seconds: -leap.diff_seconds,
                corr_seconds: corr_seconds as u32,
            })
        }
        tai_leaps.push(LeapTai {
            datetime: normalize_leap(&leap.datetime) + Duration::seconds(leap.diff_seconds),
            diff_seconds: -leap.diff_seconds,
            corr_seconds: 0,
        });
        prev_leap_diff = leap.diff_seconds;
    }
    return tai_leaps;
}

pub fn tai2utc(datetime: &str, leaps: &[LeapUtc], dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let utc = tai2utc_dt(&datetime, leaps)?;
    Ok(utc.format(dt_fmt).to_string())
}

pub fn tai2utc_dt(datetime: &NaiveDateTime, leaps: &[LeapUtc]) -> Result<NaiveDateTime, Error> {
    let leaps = utc_leaps_to_tai_leaps(leaps);
    return pick_dominant_leap(datetime, &leaps).map(|leap| {
        let mut datetime_tmp = datetime.clone();
        datetime_tmp += Duration::seconds(leap.diff_seconds);
        NaiveDateTime::from_timestamp(
            datetime_tmp.timestamp(),
            datetime_tmp.nanosecond() + leap.corr_seconds * 1_000_000_000,
        )
    });
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
    // #[case("2018-12-31T23:59:60.000", "2019-01-01T00:00:36.000")]
    // #[case("2018-12-31T23:59:61.000", "2019-01-01T00:00:37.000")]
    #[case("2019-01-01T00:00:00.000", "2019-01-01T00:00:38.000")]
    // うるう秒が2秒削除される瞬間のテスト
    #[case("2019-12-31T23:59:57.000", "2020-01-01T00:00:35.000")]
    #[case("2020-01-01T00:00:00.000", "2020-01-01T00:00:36.000")]
    fn test_tai2utc(#[case] expected_utc: &str, #[case] tai: &str) {
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
        let utc = tai2utc(&tai, &leaps, DT_FMT).unwrap();

        assert_eq!(utc, expected_utc);
    }

    #[test]
    fn test_error_on_illegal_format() {
        let utc = "2019-12-31 23:59:57.000";
        let leaps = vec![LeapUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = tai2utc(&utc, &leaps, DT_FMT);

        assert_eq!(error, Err(Error::DatetimeParseError(utc.to_string())))
    }

    #[test]
    fn test_error_on_too_low_datetime() {
        let utc = "2015-07-01T00:00:35.999";
        let leaps = vec![LeapUtc {
            datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 0),
            diff_seconds: 36,
        }];
        let error = tai2utc(&utc, &leaps, DT_FMT);

        assert_eq!(
            error,
            Err(Error::DatetimeTooLowError(
                "2015-07-01 00:00:35.999".to_string()
            ))
        )
    }
}
