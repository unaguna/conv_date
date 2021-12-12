use super::TaiUtcTable;
use crate::{error::Error, normalize_leap};
use chrono::{Duration, NaiveDateTime};

/// Difference (UTC - TAI) and the datetime at which it is applied
///
/// It expresses a row of the UTC-TAI table.
// TODO: write tests
#[derive(Debug, PartialEq)]
pub struct DiffUtcTai {
    /// (TAI) The moment when the difference (UTC - TAI) changes due to a leap second
    pub datetime: NaiveDateTime,
    /// The part of the difference (UTC - TAI) that is calculated as 60s = 1m
    pub diff_seconds: i64,
    /// The part of the difference (UTC - TAI) that is not carried forward to the minute.
    pub corr_seconds: u32,
}

/// UTC-TAI conversion table
///
/// It expresses the UTC-TAI table; it is used for conversion from TAI to UTC.
///
/// *Normally, what you need in order to use the functions of this library is [`TaiUtcTable`], and there should be no direct external use of `UtcTaiTable`.*
///
/// # As Iterable Object
///
/// It behaves as an iterable object of row.
///
/// # Creation new UtcTaiTable
///
/// This object is created from [`TaiUtcTable`]; for example:
///
/// ```
/// use convdate::convtbl::{TaiUtcTable, UtcTaiTable};
/// use chrono::NaiveDate;
///
/// let table = TaiUtcTable::from_lines(vec!["2017-01-01T00:00:00 37"], "%Y-%m-%dT%H:%M:%S").unwrap();
/// let table: UtcTaiTable = (&table).into();
/// for row in table.iter() {
///     assert_eq!(row.datetime, NaiveDate::from_ymd(2017, 1, 1).and_hms(0, 0, 37));
///     assert_eq!(row.diff_seconds, -37);
/// }
/// ```
///
pub struct UtcTaiTable {
    diff_list: Vec<DiffUtcTai>,
}

impl UtcTaiTable {
    /// Pick the row to use to calculate UTC from the TAI datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - An TAI datetime to convert to UTC
    pub fn pick_dominant_row<'a>(
        &'a self,
        datetime: &NaiveDateTime,
    ) -> Result<&'a DiffUtcTai, Error> {
        // 線形探索
        let mut prev_diff: Option<&DiffUtcTai> = None;
        for diff_utc_tai in self.iter() {
            if datetime < &diff_utc_tai.datetime {
                break;
            }
            prev_diff = Some(diff_utc_tai);
        }
        return match prev_diff {
            Some(diff_utc_tai) => Ok(diff_utc_tai),
            None => Err(Error::DatetimeTooLowError(datetime.to_string()))?,
        };
    }
}

impl From<&TaiUtcTable> for UtcTaiTable {
    fn from(tai_utc_table: &TaiUtcTable) -> Self {
        let mut diff_list = Vec::new();
        let mut prev_diff = i64::MAX;
        for diff_tai_utc in tai_utc_table.iter() {
            if prev_diff < diff_tai_utc.diff_seconds {
                let corr_seconds = diff_tai_utc.diff_seconds - prev_diff;
                diff_list.push(DiffUtcTai {
                    datetime: normalize_leap(&diff_tai_utc.datetime)
                        + Duration::seconds(diff_tai_utc.diff_seconds - corr_seconds),
                    diff_seconds: -diff_tai_utc.diff_seconds,
                    corr_seconds: corr_seconds as u32,
                })
            }
            diff_list.push(DiffUtcTai {
                datetime: normalize_leap(&diff_tai_utc.datetime)
                    + Duration::seconds(diff_tai_utc.diff_seconds),
                diff_seconds: -diff_tai_utc.diff_seconds,
                corr_seconds: 0,
            });
            prev_diff = diff_tai_utc.diff_seconds;
        }
        return UtcTaiTable { diff_list };
    }
}

impl std::ops::Deref for UtcTaiTable {
    type Target = [DiffUtcTai];
    fn deref(&self) -> &[DiffUtcTai] {
        self.diff_list.deref()
    }
}
