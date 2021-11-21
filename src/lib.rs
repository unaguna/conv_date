use chrono::{
    Date, DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc,
};

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    datetime: DateTime<Utc>,
    // うるう秒による累積のずれ (TAI - UTC)
    diff_seconds: i64,
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

pub fn utc2tai(datetime: &DateTime<Utc>, leaps: &[LeapUtc]) -> Result<NaiveDateTime, String> {
    let datetime_nm = normalize_leap(datetime);

    return pick_dominant_leap(datetime, leaps)
        .map(|leap| datetime_nm + Duration::seconds(leap.diff_seconds));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

    #[rstest]
    #[case("2016-12-31T23:59:59.000", "2017-01-01T00:00:35.000")]
    #[case("2016-12-31T23:59:60.000", "2017-01-01T00:00:36.000")]
    #[case("2016-12-31T23:59:60.123", "2017-01-01T00:00:36.123")]
    #[case("2017-01-01T00:00:00.000", "2017-01-01T00:00:37.000")]
    #[case("2017-01-02T11:22:33.000", "2017-01-02T11:23:10.000")]
    #[case("2017-01-02T11:22:33.123", "2017-01-02T11:23:10.123")]
    fn it_works(#[case] utc: &str, #[case] expected_tai: &str) {
        let utc = Utc.datetime_from_str(utc, DT_FMT).unwrap();
        let expected_tai = NaiveDateTime::parse_from_str(expected_tai, DT_FMT).unwrap();

        let leaps = vec![
            LeapUtc {
                datetime: Utc.ymd(2015, 7, 1).and_hms(0, 0, 0),
                diff_seconds: 36,
            },
            LeapUtc {
                datetime: Utc.ymd(2017, 1, 1).and_hms(0, 0, 0),
                diff_seconds: 37,
            },
        ];
        let tai = utc2tai(&utc, &leaps).unwrap();

        assert_eq!(tai, expected_tai);
    }
}
