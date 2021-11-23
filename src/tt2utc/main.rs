use conv_date::{exe, tai2utc, tt2tai};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from TT to UTC");

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap();

    // calc UTC
    for in_tt in args.get_datetimes() {
        let utc = tt2tai(in_tt, args.get_dt_fmt())
            .and_then(|tai| tai2utc(&tai, &leaps, args.get_dt_fmt()))
            .unwrap();

        println!("{}", utc)
    }
}
