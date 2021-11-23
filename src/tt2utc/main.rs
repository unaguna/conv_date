use conv_date::{exe, tai2utc, tt2tai};
use std::env;

fn main() {
    // Analize the arguments
    // TODO: error checking
    let args: Vec<String> = env::args().collect();
    let in_tt = &args[1];

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p))
        .unwrap();
    // calc UTC
    let utc = tt2tai(in_tt).and_then(|tai| tai2utc(&tai, &leaps)).unwrap();

    println!("{}", utc)
}
