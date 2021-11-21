use chrono::{TimeZone, Utc};
use conv_date::{utc2tai, LeapUtc};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S";

use std::env;

fn main() {
    // Analize the arguments
    // TODO: error checking
    let args: Vec<String> = env::args().collect();
    let in_utc = &args[1];
    let leaps_file = &args[2];

    // load leap list
    let leaps: Vec<_> = BufReader::new(File::open(leaps_file).unwrap())
        .lines()
        .map(|line| line2leap(&line.unwrap(), " ", DT_FMT).unwrap())
        .collect();
    let tai = utc2tai(in_utc, &leaps).unwrap();

    println!("{}", tai)
}

fn line2leap(line: &str, sep: &str, fmt: &str) -> Result<LeapUtc, String> {
    let parts: Vec<&str> = line.splitn(3, sep).collect();
    if parts.len() != 2 {
        return Err(format!("Illegal leap definition (block size): {}", line));
    }

    let datetime = Utc.datetime_from_str(parts[0], fmt);
    let datetime = match datetime {
        Ok(datetime) => datetime,
        Err(_e) => {
            return Err(format!(
                "Illegal leap definition (datetime format): {}",
                line
            ))
        }
    };

    let diff_seconds: Result<i64, _> = parts[1].parse();
    let diff_seconds = match diff_seconds {
        Ok(diff_seconds) => diff_seconds,
        Err(_e) => return Err(format!("Illegal leap definition (delta seconds): {}", line)),
    };

    Ok(LeapUtc {
        datetime,
        diff_seconds,
    })
}
