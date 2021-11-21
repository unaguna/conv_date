use chrono::{Date, DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    datetime: DateTime<Utc>,
    // うるう秒による累積のずれ (TAI - UTC)
    diff_seconds: i32,
}

pub fn utc2tai(datetime: &DateTime<Utc>, leaps: &[LeapUtc]) -> Result<NaiveDateTime, String> {
    panic!("Not implemented.")
}

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
