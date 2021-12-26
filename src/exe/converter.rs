use super::*;
use crate::error::Error;
use std::io::{BufRead, Write};

pub trait Converter {
    fn convert(&self, datetime: &str) -> Result<String, Error>;
}

pub fn main_convertion<C: Converter>(
    converter: &C,
    args: &Arguments,
    params: &Parameters,
    stdin: &mut impl BufRead,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> i32 {
    // function for output to stdout
    let print_line = get_print_line(&params);

    // Chooses input datetimes stream
    let dt_stream: Box<dyn Iterator<Item = Result<String, _>>> = match args.get_datetimes() {
        Some(datetimes) => Box::new(datetimes.map(|s| Ok(s.to_string()))),
        None => Box::new(stdin.lines()),
    };

    // calc UTC
    let mut someone_is_err = false;
    for in_dt in dt_stream {
        let in_dt = match in_dt {
            Ok(in_dt) => in_dt,
            Err(e) => {
                someone_is_err = true;
                print_err(stderr, &e);

                // This error occurs when the input stream is invalid.
                // In other words, subsequent inputs are also likely to be abnormal,
                // so the process is terminated without `continue`.
                break;
            }
        };

        let out_dt = converter.convert(&in_dt);

        match out_dt {
            Err(e) => {
                someone_is_err = true;
                print_err(stderr, &e)
            }
            Ok(out_dt) => print_line(stdout, &in_dt, &out_dt),
        }
    }

    return if someone_is_err {
        EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        EXIT_CODE_OK
    };
}
