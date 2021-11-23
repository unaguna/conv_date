use conv_date::{exe, utc2tai};

use std::env;

fn main() {
    // Analize the arguments
    // TODO: error checking
    let args: Vec<String> = env::args().collect();
    let in_utc = &args[1];

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p))
        .unwrap();
    let tai = utc2tai(in_utc, &leaps).unwrap();

    println!("{}", tai)
}
