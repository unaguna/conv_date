use super::Converter;
use crate::convtbl::TaiUtcTable;
use crate::error::Error;
use crate::utc2tai;

pub struct Utc2TaiConverter<'a> {
    table: TaiUtcTable,
    dt_fmt: &'a str,
}

impl Utc2TaiConverter<'_> {
    pub fn new(table: TaiUtcTable, dt_fmt: &str) -> Utc2TaiConverter {
        Utc2TaiConverter { table, dt_fmt }
    }
}

impl Converter for Utc2TaiConverter<'_> {
    fn convert(&self, datetime: &str) -> Result<String, Error> {
        utc2tai(datetime, &self.table, &self.dt_fmt)
    }
}
