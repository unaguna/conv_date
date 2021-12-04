use crate::{error::Error, LeapUtc};
use crate::{tai2tt_dt, utc2tai_dt};
use anyhow::Result;
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

// TODO: unit test
