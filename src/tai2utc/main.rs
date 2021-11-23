use conv_date::{exe, tai2utc};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from TAI to UTC");

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap();

    // calc UTC
    for in_tt in args.get_datetimes() {
        let utc = tai2utc(in_tt, &leaps).unwrap();

        println!("{}", utc)
    }
}
