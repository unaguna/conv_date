use crate::{
    exe,
    exe::{Arguments, EnvValues, Parameters},
    tai2utc,
};

pub fn main_inner(
    args: impl IntoIterator<Item = String>,
    env_vars: impl IntoIterator<
        Item = (String, String),
        IntoIter = impl Iterator<Item = (String, String)>,
    >,
) {
    let args = Arguments::new("Converter from TAI to UTC", args);
    let env_vars = EnvValues::new(env_vars);

    // Analize the arguments and the environment variables.
    let params = Parameters::new(&args, &env_vars);

    // load leap list
    let leaps = exe::load_leaps(&params.get_leaps_path(), params.get_leaps_dt_fmt())
        .unwrap_or_else(|e| {
            exe::print_err(&e);
            std::process::exit(exe::EXIT_CODE_NG)
        });

    let print_line = match params.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc UTC
    let mut someone_is_err = false;
    for in_tt in args.get_datetimes() {
        let utc = tai2utc(in_tt, &leaps, params.get_dt_fmt());

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
