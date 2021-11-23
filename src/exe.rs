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
    dt_fmt: String,
    leaps_dt_fmt: String,
}

impl Arguments<'_> {
    pub fn new<'a>(app_name: &str) -> Arguments<'a> {
        let app: App<'a, 'a> = App::new(app_name)
            .arg(
                Arg::with_name("leaps_dt_fmt")
                    .help("format of datetime in leaps table file")
                    .takes_value(true)
                    .long("leaps-dt-fmt"),
            )
            .arg(
                Arg::with_name("dt_fmt")
                    .help("format of <datetime>")
                    .takes_value(true)
                    .long("dt-fmt"),
            )
            .arg(
                Arg::with_name("io_pair_flg")
                    .help("If it is specified, input datetime is also output to stdin.")
                    .short("H")
                    .long("io-pair"),
            )
            .arg(
                Arg::with_name("datetime")
                    .help("datetime to convert")
                    .multiple(true)
                    .required(true),
            );
        let matches: ArgMatches<'a> = app.get_matches();
        return Arguments {
            dt_fmt: Arguments::decide_dt_fmt(&matches),
            leaps_dt_fmt: Arguments::decide_leaps_dt_fmt(&matches),
            matches,
        };
    }

    pub fn get_datetimes(&self) -> Values {
        return self.matches.values_of("datetime").unwrap();
    }

    pub fn get_dt_fmt(&self) -> &str {
        return &self.dt_fmt;
    }

    pub fn get_leaps_dt_fmt(&self) -> &str {
        return &self.leaps_dt_fmt;
    }

    fn decide_dt_fmt(matches: &ArgMatches) -> String {
        let s: String = matches
            .value_of("dt_fmt")
            .map(|s| s.to_string())
            .or(env::var("DT_FMT").ok())
            .unwrap_or(DT_FMT.to_string());
        return s;
    }

    fn decide_leaps_dt_fmt(matches: &ArgMatches) -> String {
        let s: String = matches
            .value_of("leaps_dt_fmt")
            .map(|s| s.to_string())
            .or(env::var("LEAPS_DT_FMT").ok())
            .unwrap_or(DT_FMT.to_string());
        return s;
    }

    pub fn io_pair_flg(&self) -> bool {
        return self.matches.is_present("io_pair_flg");
    }
}
