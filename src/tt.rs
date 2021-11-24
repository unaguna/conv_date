use anyhow::Result;
use chrono::{Duration, NaiveDateTime};

const D_TT_TAI_MS: i64 = 32184;

pub fn tt2tai(datetime: &str, dt_fmt: &str) -> Result<String> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)?;
    let tai = tt2tai_dt(&datetime);
    Ok(tai.format(dt_fmt).to_string())
}

fn tt2tai_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() - Duration::milliseconds(D_TT_TAI_MS);
}

pub fn tai2tt(datetime: &str, dt_fmt: &str) -> Result<String> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)?;
    let tt = tai2tt_dt(&datetime);
    Ok(tt.format(dt_fmt).to_string())
}

fn tai2tt_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() + Duration::milliseconds(D_TT_TAI_MS);
}
