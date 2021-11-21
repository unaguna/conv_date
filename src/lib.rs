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

pub fn utc2tai(datetime: &DateTime<Utc>, leaps: &[LeapUtc]) -> Result<NaiveDateTime, String> {
    return pick_dominant_leap(datetime, leaps)
        .map(|leap| datetime.naive_utc() + Duration::seconds(leap.diff_seconds));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let utc = Utc.ymd(2017, 1, 2).and_hms(11, 22, 33);
        let leaps = vec![LeapUtc {
            datetime: Utc.ymd(2017, 1, 1).and_hms(0, 0, 0),
            diff_seconds: 37,
        }];
        let tai = utc2tai(&utc, &leaps).unwrap();

        assert_eq!(tai.year(), 2017);
        assert_eq!(tai.month(), 1);
        assert_eq!(tai.day(), 2);
        assert_eq!(tai.hour(), 11);
        assert_eq!(tai.minute(), 23);
        assert_eq!(tai.second(), 10);
        assert_eq!(tai.nanosecond(), 0);
    }
}
