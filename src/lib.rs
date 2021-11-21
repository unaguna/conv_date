use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use std::convert::TryFrom;

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    pub datetime: DateTime<Utc>,
    // うるう秒による累積のずれ (TAI - UTC)
    pub diff_seconds: i64,
}

impl LeapUtc {
    pub fn from_line(line: &str, sep: &str, fmt: &str) -> Result<LeapUtc, String> {
        let parts: Vec<&str> = line.splitn(3, sep).collect();
        if parts.len() != 2 {
            return Err(format!("Illegal leap definition (block size): {}", line));
        }
        let datetime = Utc.datetime_from_str(parts[0], fmt);
        let datetime = match datetime {
            Ok(datetime) => datetime,
            Err(_e) => {
                return Err(format!(
                    "Illegal leap definition (datetime format): {}",
                    line
                ))
            }
        };
        let diff_seconds: Result<i64, _> = parts[1].parse();
        let diff_seconds = match diff_seconds {
            Ok(diff_seconds) => diff_seconds,
            Err(_e) => return Err(format!("Illegal leap definition (delta seconds): {}", line)),
        };
        Ok(LeapUtc {
            datetime,
            diff_seconds,
        })
    }
}

pub struct LeapTai {
    // うるう秒によってずれるタイミング (TAI)
    pub datetime: NaiveDateTime,
    // うるう秒による累積のずれ (UTC - TAI) のうち、60s=1mとして計算する部分
    pub diff_seconds: i64,
    // うるう秒による累積のずれ (UTC - TAI) のうち、分に繰り上がらない部分
    pub corr_seconds: u32,
}

/// Pick the leap object to use for calc tai from the datetime.
///
/// # Arguments
///
/// * `datetime` - A datetime to convert to tai
/// * `leaps` - A list of leap objects
fn pick_dominant_leap<'a>(
    datetime: &DateTime<Utc>,
    leaps: &'a [LeapUtc],
) -> Result<&'a LeapUtc, String> {
    // 線形探索
    let mut prev_leap: Option<&LeapUtc> = None;
    for leap in leaps.iter() {
        if datetime < &leap.datetime {
            break;
        }
        prev_leap = Some(leap);
    }

    return prev_leap.ok_or(format!("The datetime is too low: {}", datetime));
}

/// Pick the leap object to use for calc utc from the datetime.
///
/// # Arguments
///
/// * `datetime` - A TAI datetime to convert to utc
/// * `leaps` - A list of leap objects
fn pick_dominant_leap_tai<'a>(
    datetime: &NaiveDateTime,
    leaps: &'a [LeapTai],
) -> Result<&'a LeapTai, String> {
    // 線形探索
    let mut prev_leap: Option<&LeapTai> = None;
    for leap in leaps.iter() {
        if datetime < &leap.datetime {
            break;
        }
        prev_leap = Some(leap);
    }

    return prev_leap.ok_or(format!("The datetime is too low: {}", datetime));
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
                corr_seconds: TryFrom::try_from(corr_seconds).unwrap(),
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

/// Convert datetime to naive without leap
///
/// Nanoseconds that exceed 1000000 to represent leap seconds are added to seconds.
fn normalize_leap(datetime: &DateTime<Utc>) -> NaiveDateTime {
    return NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day()).and_hms(
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    ) + Duration::nanoseconds(datetime.nanosecond().into());
}

pub fn utc2tai(datetime: &str, leaps: &[LeapUtc]) -> Result<String, String> {
    Utc.datetime_from_str(datetime, DT_FMT)
        .map_err(|err| err.to_string())
        .and_then(|datetime| utc2tai_dt(&datetime, leaps))
        .map(|tai| tai.format(DT_FMT).to_string())
}

pub fn utc2tai_dt(datetime: &DateTime<Utc>, leaps: &[LeapUtc]) -> Result<NaiveDateTime, String> {
    let datetime_nm = normalize_leap(datetime);

    return pick_dominant_leap(datetime, leaps)
        .map(|leap| datetime_nm + Duration::seconds(leap.diff_seconds));
}

pub fn tai2utc(datetime: &str, leaps: &[LeapUtc]) -> Result<String, String> {
    NaiveDateTime::parse_from_str(datetime, DT_FMT)
        .map_err(|err| err.to_string())
        .and_then(|datetime| tai2utc_dt(&datetime, leaps))
        .map(|utc| utc.format(DT_FMT).to_string())
}

pub fn tai2utc_dt(datetime: &NaiveDateTime, leaps: &[LeapUtc]) -> Result<NaiveDateTime, String> {
    let leaps = utc_leaps_to_tai_leaps(leaps);

    return pick_dominant_leap_tai(datetime, &leaps).map(|leap| {
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
    use chrono::TimeZone;
    use rstest::*;

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
        let leaps = vec![
            LeapUtc {
                datetime: Utc.ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: Utc.ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
            LeapUtc {
                datetime: Utc.ymd(2018, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 38,
            },
            LeapUtc {
                datetime: Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
        ];
        let tai = utc2tai(&utc, &leaps).unwrap();

        assert_eq!(tai, expected_tai);
    }

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
                datetime: Utc.ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: Utc.ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
            LeapUtc {
                datetime: Utc.ymd(2018, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 38,
            },
            LeapUtc {
                datetime: Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
        ];
        let utc = tai2utc(&tai, &leaps).unwrap();

        assert_eq!(utc, expected_utc);
    }
}
