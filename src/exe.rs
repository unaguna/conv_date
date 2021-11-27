use crate::{LeapUtc, DT_FMT};
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, Values};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
pub mod tai2utc;
pub mod tt2utc;
pub mod utc2tai;
pub mod utc2tt;

const LEAPS_TABLE_FILENAME: &str = "leaps.txt";
pub const EXIT_CODE_OK: i32 = 0;
pub const EXIT_CODE_NG: i32 = 1;
pub const EXIT_CODE_SOME_DT_NOT_CONVERTED: i32 = 2;

pub fn print_err(err: &dyn std::fmt::Display) {
    eprintln!("{}: {}", exe_name(), err)
}

pub fn exe_name() -> String {
    return PathBuf::from(env::args().next().unwrap())
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_string();
}

pub fn load_leaps(leaps_file: &PathBuf, datetime_fmt: &str) -> Result<Vec<LeapUtc>> {
    let leaps_file = File::open(leaps_file).context(format!(
        "The leaps table file isn't available: {}",
        leaps_file.to_str().unwrap()
    ))?;

    let leaps: Result<Vec<_>, _> = BufReader::new(leaps_file)
        .lines()
        .map(|line| LeapUtc::from_line(&line?, " ", datetime_fmt))
        .collect();
    leaps
}

/// Command arguments of conv_date
pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
    leaps_dt_fmt: Option<String>,
    dt_fmt: Option<String>,
    io_pair_flg: bool,
    leaps_path: Option<String>,
}

impl Arguments<'_> {
    pub fn new<'a>(app_name: &str, args: impl IntoIterator<Item = String>) -> Arguments<'a> {
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
                Arg::with_name("leaps_table_file")
                    .help("Filepath of leaps table file. If it is not specified, environment value 'LEAPS_TABLE' is used. If both of them are not specified, the default file is used.")
                    .takes_value(true)
                    .long("leaps-table"),
            )
            .arg(
                Arg::with_name("datetime")
                    .help("datetime to convert")
                    .multiple(true)
                    .required(true),
            );
        let matches: ArgMatches<'a> = app.get_matches_from(args);
        Arguments::<'a> {
            leaps_dt_fmt: matches.value_of("leaps_dt_fmt").map(|s| s.to_string()),
            dt_fmt: matches.value_of("dt_fmt").map(|s| s.to_string()),
            io_pair_flg: matches.is_present("io_pair_flg"),
            leaps_path: matches.value_of("leaps_table_file").map(|s| s.to_string()),
            matches: matches,
        }
    }

    pub fn get_dt_fmt(&self) -> Option<&str> {
        self.dt_fmt.as_ref().map(|s| s.as_str())
    }

    pub fn get_leaps_dt_fmt(&self) -> Option<&str> {
        self.leaps_dt_fmt.as_ref().map(|s| s.as_str())
    }

    pub fn get_leaps_path(&self) -> Option<&str> {
        self.leaps_path.as_ref().map(|s| s.as_str())
    }

    pub fn get_io_pair_flg(&self) -> bool {
        self.io_pair_flg
    }

    pub fn get_datetimes(&self) -> Values {
        // It can unwrap because "datetime" is required.
        return self.matches.values_of("datetime").unwrap();
    }
}

/// Environment variables which conv_date uses
pub struct EnvValues {
    dt_fmt: Option<String>,
    leaps_dt_fmt: Option<String>,
    leaps_path: Option<String>,
}

impl EnvValues {
    pub fn new(
        iter: impl IntoIterator<
            Item = (String, String),
            IntoIter = impl Iterator<Item = (String, String)>,
        >,
    ) -> EnvValues {
        let map = iter.into_iter().collect::<HashMap<_, _>>();
        EnvValues {
            dt_fmt: map.get("DT_FMT").map(|s| s.to_string()),
            leaps_dt_fmt: map.get("LEAPS_DT_FMT").map(|s| s.to_string()),
            leaps_path: map.get("LEAPS_TABLE").map(|s| s.to_string()),
        }
    }

    pub fn get_dt_fmt(&self) -> Option<&str> {
        self.dt_fmt.as_ref().map(|s| s.as_str())
    }

    pub fn get_leaps_dt_fmt(&self) -> Option<&str> {
        self.leaps_dt_fmt.as_ref().map(|s| s.as_str())
    }

    pub fn get_leaps_path(&self) -> Option<&str> {
        self.leaps_path.as_ref().map(|s| s.as_str())
    }
}

pub struct Parameters<'a> {
    dt_fmt: &'a str,
    leaps_dt_fmt: &'a str,
    leaps_path: PathBuf,
    io_pair_flg: bool,
}

impl Parameters<'_> {
    pub fn new<'a>(args: &'a Arguments, env_vars: &'a EnvValues) -> Parameters<'a> {
        return Parameters {
            dt_fmt: Parameters::decide_dt_fmt(args, env_vars),
            leaps_dt_fmt: Parameters::decide_leaps_dt_fmt(args, env_vars),
            leaps_path: Parameters::decide_leaps_path(args, env_vars),
            io_pair_flg: args.io_pair_flg,
        };
    }

    pub fn get_dt_fmt(&self) -> &str {
        return &self.dt_fmt;
    }

    pub fn get_leaps_dt_fmt(&self) -> &str {
        return &self.leaps_dt_fmt;
    }

    fn decide_dt_fmt<'a>(args: &'a Arguments, env_vars: &'a EnvValues) -> &'a str {
        args.get_dt_fmt()
            .or_else(|| env_vars.get_dt_fmt())
            .unwrap_or(DT_FMT)
    }

    fn decide_leaps_dt_fmt<'a>(args: &'a Arguments, env_vars: &'a EnvValues) -> &'a str {
        args.get_leaps_dt_fmt()
            .or_else(|| env_vars.get_leaps_dt_fmt())
            .unwrap_or(DT_FMT)
    }

    pub fn io_pair_flg(&self) -> bool {
        return self.io_pair_flg;
    }

    pub fn get_leaps_path(&self) -> &PathBuf {
        return &self.leaps_path;
    }

    fn decide_leaps_path(args: &Arguments, env_vars: &EnvValues) -> PathBuf {
        // If it is specified as command args, use it.
        if let Some(path) = args.get_leaps_path() {
            return PathBuf::from(path);
        }

        // If it is spcified as environment variable, use it.
        if let Some(path) = env_vars.get_leaps_path() {
            return PathBuf::from(path);
        }

        // use default file
        let mut exe_path = env::current_exe().unwrap();
        exe_path.pop();
        exe_path.push(LEAPS_TABLE_FILENAME);
        return exe_path;
    }
}
