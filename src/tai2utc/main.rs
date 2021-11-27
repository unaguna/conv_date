use conv_date::{exe, tai2utc};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from TAI to UTC");

    // load leap list
    let leaps =
        exe::load_leaps(&args.get_leaps_path(), args.get_leaps_dt_fmt()).unwrap_or_else(|e| {
            exe::print_err(&e);
            std::process::exit(exe::EXIT_CODE_NG)
        });

    let print_line = match args.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc UTC
    let mut someone_is_err = false;
    for in_tt in args.get_datetimes() {
        let utc = tai2utc(in_tt, &leaps, args.get_dt_fmt());

        match utc {
            Err(e) => {
                someone_is_err = true;
                exe::print_err(&e)
            }
            Ok(utc) => print_line(in_tt, &utc),
        }
    }

    std::process::exit(if someone_is_err {
        exe::EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        exe::EXIT_CODE_OK
    });
}
