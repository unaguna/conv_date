use crate::error::Error;
use chrono::{Duration, NaiveDateTime};

const D_TT_TAI_MS: i64 = 32184;

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time).
///
/// # Arguments
/// * `datetime` - Datetime in TT.
/// * `dt_fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of `datetime`
///
/// # Returns
/// Returns the datetime in TAI.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate;
///
/// let tai = convdate::tt2tai(
///     "2017-01-01T12:00:00.000",
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(tai, Ok("2017-01-01T11:59:27.816".to_string()));
/// ```
pub fn tt2tai(datetime: &str, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tai = tt2tai_dt(&datetime);
    Ok(tai.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
/// to [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time).
///
/// # Arguments
/// * `datetime` - Datetime in TT.
///
/// # Returns
/// Returns the datetime in TAI.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # See also
/// * [`tt2tai`] - It is same as `tt2tai_dt`, except that the argument and the result are [`str`] and [`String`].
pub fn tt2tai_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() - Duration::milliseconds(D_TT_TAI_MS);
}

/// Convert datetime
/// from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
/// to [TT](https://en.wikipedia.org/wiki/Terrestrial_Time).
///
/// # Arguments
/// * `datetime` - Datetime in TAI.
/// * `dt_fmt` - [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of `datetime`
///
/// # Returns
/// Returns the datetime in TT.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # Examples
/// ```
/// use convdate;
///
/// let tai = convdate::tai2tt(
///     "2017-01-01T12:00:00.000",
///     "%Y-%m-%dT%H:%M:%S%.3f");
///
/// assert_eq!(tai, Ok("2017-01-01T12:00:32.184".to_string()));
/// ```
pub fn tai2tt(datetime: &str, dt_fmt: &str) -> Result<String, Error> {
    let datetime = NaiveDateTime::parse_from_str(datetime, dt_fmt)
        .map_err(|_e| Error::DatetimeParseError(datetime.to_string()))?;
    let tt = tai2tt_dt(&datetime);
    Ok(tt.format(dt_fmt).to_string())
}

/// Convert datetime
/// from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
/// to [TT](https://en.wikipedia.org/wiki/Terrestrial_Time).
///
/// # Arguments
/// * `datetime` - Datetime in TAI.
///
/// # Returns
/// Returns the datetime in TT.
///
/// Returns [`Error`](crate::error::Error) if it fail to convert.
///
/// # See also
/// * [`tai2tt`] - It is same as `tai2tt_dt`, except that the argument and the result are [`str`] and [`String`].
pub fn tai2tt_dt(datetime: &NaiveDateTime) -> NaiveDateTime {
    return datetime.clone() + Duration::milliseconds(D_TT_TAI_MS);
}
