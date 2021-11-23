use crate::{LeapUtc, DT_FMT};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const LEAPS_TABLE_FILENAME: &str = "leaps.txt";

pub fn get_leaps_path() -> Result<PathBuf, std::io::Error> {
    let mut exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => return Err(e),
    };
    exe_path.pop();
    exe_path.push(LEAPS_TABLE_FILENAME);
    return Ok(exe_path);
}

pub fn load_leaps(leaps_file: &PathBuf) -> Result<Vec<LeapUtc>, std::io::Error> {
    let leaps: Vec<_> = BufReader::new(File::open(leaps_file).unwrap())
        .lines()
        .map(|line| LeapUtc::from_line(&line.unwrap(), " ", DT_FMT).unwrap())
        .collect();
    Ok(leaps)
}
