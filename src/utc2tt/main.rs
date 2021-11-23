use conv_date::{exe, tai2tt, utc2tai};
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
    // calc UTC
    let tt = utc2tai(in_utc, &leaps)
        .and_then(|tai| tai2tt(&tai))
        .unwrap();

    println!("{}", tt)
}
