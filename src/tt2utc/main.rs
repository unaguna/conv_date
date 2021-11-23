use conv_date::{tai2utc, tt2tai, LeapUtc};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DT_FMT: &str = "%Y-%m-%dT%H:%M:%S";

use std::env;

fn main() {
    // Analize the arguments
    // TODO: error checking
    let args: Vec<String> = env::args().collect();
    let in_tt = &args[1];
    let leaps_file = &args[2];

    // load leap list
    let leaps: Vec<_> = BufReader::new(File::open(leaps_file).unwrap())
        .lines()
        .map(|line| LeapUtc::from_line(&line.unwrap(), " ", DT_FMT).unwrap())
        .collect();
    // calc UTC
    let utc = tt2tai(in_tt).and_then(|tai| tai2utc(&tai, &leaps)).unwrap();

    println!("{}", utc)
}
