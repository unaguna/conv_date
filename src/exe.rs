use crate::{LeapUtc, DT_FMT};
use clap::{App, Arg, ArgMatches, Values};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const LEAPS_TABLE_FILENAME: &str = "leaps.txt";

pub fn get_leaps_path() -> Result<PathBuf, String> {
    let mut exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => return Err(e.to_string()),
    };
    exe_path.pop();
    exe_path.push(LEAPS_TABLE_FILENAME);
    return Ok(exe_path);
}

pub fn load_leaps(leaps_file: &PathBuf, datetime_fmt: &str) -> Result<Vec<LeapUtc>, String> {
    let leaps_file = File::open(leaps_file);
    let leaps_file = match leaps_file {
        Ok(leaps_file) => leaps_file,
        Err(err) => return Err(err.to_string()),
    };

    let leaps: Result<Vec<_>, _> = BufReader::new(leaps_file)
        .lines()
        .map(|line| match line {
            Ok(line) => LeapUtc::from_line(&line, " ", datetime_fmt),
            Err(err) => Err(err.to_string()),
        })
        .collect();
    leaps
}

pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
}

impl Arguments<'_> {
    pub fn new<'a>(app_name: &str) -> Arguments<'a> {
        let app: App<'a, 'a> = App::new(app_name)
            .arg(
                Arg::with_name("leaps_dt_fmt")
                    .help("format of datetime in leaps table file")
                    .long("leaps-dt-fmt")
                    .default_value(DT_FMT),
            )
            .arg(
                Arg::with_name("datetime")
                    .help("datetime to convert")
                    .multiple(true)
                    .required(true),
            );
        let matches: ArgMatches<'a> = app.get_matches();
        return Arguments { matches };
    }

    pub fn get_datetimes(&self) -> Values {
        return self.matches.values_of("datetime").unwrap();
    }

    pub fn get_leaps_dt_fmt(&self) -> &str {
        return self.matches.value_of("leaps_dt_fmt").unwrap();
    }
}
