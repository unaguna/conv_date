use chrono::{Duration, NaiveDateTime};

const D_TT_TAI_MS: i64 = 32184;

pub fn tt2tai(datetime: &str, dt_fmt: &str) -> Result<String, String> {
    NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|err| err.to_string())
        .map(|datetime| tt2tai_dt(&datetime))
        .map(|utc| utc.format(dt_fmt).to_string())
}

fn tt2tai_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() - Duration::milliseconds(D_TT_TAI_MS);
}

pub fn tai2tt(datetime: &str, dt_fmt: &str) -> Result<String, String> {
    NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|err| err.to_string())
        .map(|datetime| tai2tt_dt(&datetime))
        .map(|utc| utc.format(dt_fmt).to_string())
}

fn tai2tt_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() + Duration::milliseconds(D_TT_TAI_MS);
}
