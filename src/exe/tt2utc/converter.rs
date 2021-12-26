use super::Converter;
use crate::convtbl::UtcTaiTable;
use crate::error::Error;
use crate::tt2utc;

pub struct Tt2UtcConverter<'a> {
    table: UtcTaiTable,
    dt_fmt: &'a str,
}

impl Tt2UtcConverter<'_> {
    pub fn new(table: UtcTaiTable, dt_fmt: &str) -> Tt2UtcConverter {
        Tt2UtcConverter { table, dt_fmt }
    }
}

impl Converter for Tt2UtcConverter<'_> {
    fn convert(&self, datetime: &str) -> Result<String, Error> {
        tt2utc(datetime, &self.table, &self.dt_fmt)
    }
}
