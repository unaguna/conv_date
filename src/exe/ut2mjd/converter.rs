use super::Converter;
use crate::error::Error;
use crate::ut2mjd_str;

pub struct Ut2MjdConverter<'a> {
    dt_fmt: &'a str,
}

impl Ut2MjdConverter<'_> {
    pub fn new(dt_fmt: &str) -> Ut2MjdConverter {
        Ut2MjdConverter { dt_fmt }
    }
}

impl Converter for Ut2MjdConverter<'_> {
    fn convert(&self, datetime: &str) -> Result<String, Error> {
        ut2mjd_str(datetime, &self.dt_fmt)
    }
}
