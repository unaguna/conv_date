use conv_date::{tai2utc, LeapUtc};
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
        .map(|line| LeapUtc::from_line(&line.unwrap(), " ", DT_FMT).unwrap())
        .collect();
    let tai = tai2utc(in_utc, &leaps).unwrap();

    println!("{}", tai)
}
