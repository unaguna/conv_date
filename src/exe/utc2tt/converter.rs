use super::Converter;
use crate::convtbl::TaiUtcTable;
use crate::error::Error;
use crate::utc2tt;

pub struct Utc2TtConverter<'a> {
    table: TaiUtcTable,
    dt_fmt: &'a str,
}

impl Utc2TtConverter<'_> {
    pub fn new(table: TaiUtcTable, dt_fmt: &str) -> Utc2TtConverter {
        Utc2TtConverter { table, dt_fmt }
    }
}

impl Converter for Utc2TtConverter<'_> {
    fn convert(&self, datetime: &str) -> Result<String, Error> {
        utc2tt(datetime, &self.table, &self.dt_fmt)
    }
}
