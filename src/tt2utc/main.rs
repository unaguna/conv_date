use conv_date::{exe, tai2utc, tt2tai};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from TT to UTC");

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap();

    let print_line = match args.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc UTC
    for in_tt in args.get_datetimes() {
        let utc = tt2tai(in_tt, args.get_dt_fmt())
            .and_then(|tai| tai2utc(&tai, &leaps, args.get_dt_fmt()))
            .unwrap();

        print_line(in_tt, &utc);
    }
}
