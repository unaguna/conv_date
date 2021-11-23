use conv_date::{exe, tai2tt, utc2tai};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from UTC to TT");

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap();

    // calc TT
    for in_utc in args.get_datetimes() {
        let tt = utc2tai(in_utc, &leaps, args.get_dt_fmt())
            .and_then(|tai| tai2tt(&tai, args.get_dt_fmt()))
            .unwrap();

        println!("{}", tt)
    }
}
