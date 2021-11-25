use conv_date::{exe, utc2tai};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from UTC to TAI");

    // load leap list
    let leaps = exe::get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap();

    let print_line = match args.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc TAI
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tai = utc2tai(in_utc, &leaps, args.get_dt_fmt());

        match tai {
            Err(e) => {
                someone_is_err = true;
                eprintln!("{}", e)
            }
            Ok(tai) => print_line(in_utc, &tai),
        }
    }

    std::process::exit(if someone_is_err {
        exe::EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        exe::EXIT_CODE_OK
    });
}
