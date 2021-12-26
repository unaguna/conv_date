use super::Converter;
use crate::convtbl::UtcTaiTable;
use crate::error::Error;
use crate::tai2utc;

pub struct Tai2UtcConverter<'a> {
    table: UtcTaiTable,
    dt_fmt: &'a str,
}

impl Tai2UtcConverter<'_> {
    pub fn new(table: UtcTaiTable, dt_fmt: &str) -> Tai2UtcConverter {
        Tai2UtcConverter { table, dt_fmt }
    }
}

impl Converter for Tai2UtcConverter<'_> {
    fn convert(&self, datetime: &str) -> Result<String, Error> {
        tai2utc(datetime, &self.table, &self.dt_fmt)
    }
}
