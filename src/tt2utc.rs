use crate::{error::Error, LeapUtc};
use crate::{tai2utc_dt, tt2tai_dt};
use anyhow::Result;
use chrono::NaiveDateTime;

pub fn tt2utc(datetime: &str, leaps: &[LeapUtc], dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = tt2utc_dt(&datetime, leaps)?;
    Ok(tai.format(dt_fmt).to_string())
}

pub fn tt2utc_dt(datetime: &NaiveDateTime, leaps: &[LeapUtc]) -> Result<NaiveDateTime, Error> {
    let tai = tt2tai_dt(datetime);

    match tai2utc_dt(&tai, leaps) {
        Err(Error::DatetimeTooLowError(_)) => {
            // 多段階で変換を行う場合、中間の日時文字列がエラーメッセージに使われている場合があるため、入力された日時文字列に置き換える。
            Err(Error::DatetimeTooLowError(datetime.to_string()))
        }
        Err(e) => Err(e),
        Ok(utc) => Ok(utc),
    }
}

// TODO: unit test
