use conv_date::{exe, exe::EnvValues, utc2tai};
use std::env;

fn main() {
    main_inner(env::vars())
}

fn main_inner(
    env_vars: impl IntoIterator<
        Item = (String, String),
        IntoIter = impl Iterator<Item = (String, String)>,
    >,
) {
    let env_vars = EnvValues::new(env_vars);

    // Analize the arguments
    let args = exe::Parameters::new("Converter from UTC to TAI", &env_vars);

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

    // calc TAI
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tai = utc2tai(in_utc, &leaps, args.get_dt_fmt());

        match tai {
            Err(e) => {
                someone_is_err = true;
                exe::print_err(&e)
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
