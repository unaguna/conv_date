use crate::{LeapUtc, DT_FMT};
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

pub fn load_leaps(leaps_file: &PathBuf) -> Result<Vec<LeapUtc>, String> {
    let leaps_file = File::open(leaps_file);
    let leaps_file = match leaps_file {
        Ok(leaps_file) => leaps_file,
        Err(err) => return Err(err.to_string()),
    };

    let leaps: Result<Vec<_>, _> = BufReader::new(leaps_file)
        .lines()
        .map(|line| match line {
            Ok(line) => LeapUtc::from_line(&line, " ", DT_FMT),
            Err(err) => Err(err.to_string()),
        })
        .collect();
    leaps
}
