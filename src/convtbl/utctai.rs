use super::TaiUtcTable;
use crate::{error::Error, normalize_leap};
use chrono::{Duration, NaiveDateTime};

/// Difference (UTC - TAI) and the datetime at which it is applied
///
/// It expresses a row of the UTC-TAI table.
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
#[derive(Debug)]
pub struct UtcTaiTable(Vec<DiffUtcTai>);

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
        return UtcTaiTable(diff_list);
    }
}

impl std::ops::Deref for UtcTaiTable {
    type Target = [DiffUtcTai];
    fn deref(&self) -> &[DiffUtcTai] {
        self.0.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convtbl::TaiUtcTable;
    use crate::testmod;
    use chrono::{NaiveDate, NaiveDateTime};
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(
        NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 34),
        None,
        Some(Error::DatetimeTooLowError("2012-07-01 00:00:34".to_string())),
    )]
    #[case(
        NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 35),
        Some(DiffUtcTai{datetime: NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 35), diff_seconds: -35, corr_seconds: 0}),
        None,
    )]
    #[case(
        NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 34),
        Some(DiffUtcTai{datetime: NaiveDate::from_ymd(2012, 7, 1).and_hms(0, 0, 35), diff_seconds: -35, corr_seconds: 0}),
        None,
    )]
    #[case(
        NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 35),
        Some(DiffUtcTai{datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 35), diff_seconds: -36, corr_seconds: 1}),
        None,
    )]
    #[case(
        NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 36),
        Some(DiffUtcTai{datetime: NaiveDate::from_ymd(2015, 7, 1).and_hms(0, 0, 36), diff_seconds: -36, corr_seconds: 0}),
        None,
    )]
    fn test_pick_dominant_row<'a>(
        #[case] dt_input: NaiveDateTime,
        #[case] expected_ok: Option<DiffUtcTai>,
        #[case] expected_err: Option<Error>,
    ) {
        let expected = testmod::result(expected_ok.as_ref(), expected_err);

        let tai_utc_table = TaiUtcTable::from_lines(
            vec!["20120701000000 35", "20150701000000 36"],
            "%Y%m%d%H%M%S",
        )
        .unwrap();
        let utc_tai_table = UtcTaiTable::from(&tai_utc_table);

        let dominant_row = utc_tai_table.pick_dominant_row(&dt_input);

        assert_eq!(dominant_row, expected);
    }

    /// Tests construction of UtcTaiTable from TaiUtcTable.
    #[test]
    fn test_from_tai_utc_table() {
        let tai_utc_table = TaiUtcTable::from_lines(
            vec![
                "20120701000000 35",
                "20150701000000 36",
                "20170101000000 35",
            ],
            "%Y%m%d%H%M%S",
        )
        .unwrap();

        let utc_tai_table = UtcTaiTable::from(&tai_utc_table);

        let diff_utc_tai_list: Vec<&DiffUtcTai> = utc_tai_table.iter().collect();
        assert_eq!(diff_utc_tai_list.len(), 4);
        assert_eq!(
            diff_utc_tai_list[0].datetime,
            NaiveDateTime::from_str("2012-07-01T00:00:35").unwrap()
        );
        assert_eq!(diff_utc_tai_list[0].diff_seconds, -35);
        assert_eq!(diff_utc_tai_list[0].corr_seconds, 0);
        assert_eq!(
            diff_utc_tai_list[1].datetime,
            NaiveDateTime::from_str("2015-07-01T00:00:35").unwrap()
        );
        assert_eq!(diff_utc_tai_list[1].diff_seconds, -36);
        assert_eq!(diff_utc_tai_list[1].corr_seconds, 1);
        assert_eq!(
            diff_utc_tai_list[2].datetime,
            NaiveDateTime::from_str("2015-07-01T00:00:36").unwrap()
        );
        assert_eq!(diff_utc_tai_list[2].diff_seconds, -36);
        assert_eq!(diff_utc_tai_list[2].corr_seconds, 0);
        assert_eq!(
            diff_utc_tai_list[3].datetime,
            NaiveDateTime::from_str("2017-01-01T00:00:35").unwrap()
        );
        assert_eq!(diff_utc_tai_list[3].diff_seconds, -35);
        assert_eq!(diff_utc_tai_list[3].corr_seconds, 0);
    }
}
