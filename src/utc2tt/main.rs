use clap::{App, Arg};
use conv_date::{exe, tai2tt, utc2tai};

fn main() {
    // Analize the arguments
    let app = App::new("utc2tt").arg(
        Arg::with_name("datetime")
            .help("datetime to convert")
            .multiple(true)
            .required(true),
    );
    let matches = app.get_matches();

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p))
        .unwrap();
    // calc UTC
    for in_utc in matches.values_of("datetime").unwrap() {
        let tt = utc2tai(in_utc, &leaps)
            .and_then(|tai| tai2tt(&tai))
            .unwrap();

        println!("{}", tt)
    }
}
