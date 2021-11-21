use chrono::{Date, Datelike, DateTime, NaiveDate, NaiveDateTime, Timelike, TimeZone, Utc};

pub struct LeapUtc {
    // うるう秒によってずれるタイミング (UTC)
    datetime: NaiveDateTime,
    // うるう秒による累積のずれ (TAI - UTC)
    diff_seconds: i32,
}

pub fn utc2tai(datetime: DateTime<Utc>, leaps: Vec<LeapUtc>) -> NaiveDateTime {
    panic!()
}

#[test]
fn it_works() {
    let utc = Utc.ymd(2017, 1, 2).and_hms(11, 22, 33);
    let leaps = vec![LeapUtc { datetime: NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 0), diff_seconds: 37 }];
    let tai = utc2tai(utc, leaps);

    assert_eq!(tai.year(), 2017);
    assert_eq!(tai.month(), 1);
    assert_eq!(tai.day(), 2);
    assert_eq!(tai.hour(), 11);
    assert_eq!(tai.minute(), 23);
    assert_eq!(tai.second(), 10);
    assert_eq!(tai.nanosecond(), 0);
}
