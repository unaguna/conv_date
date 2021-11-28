use super::{Arguments, EnvValues, Parameters};
use crate::{error::Error, exe, tai2tt, utc2tai};

pub fn main_inner(
    args: impl IntoIterator<Item = String>,
    env_vars: impl IntoIterator<
        Item = (String, String),
        IntoIter = impl Iterator<Item = (String, String)>,
    >,
) -> i32 {
    let args = Arguments::new("Converter from UTC to TT", args);
    let env_vars = EnvValues::new(env_vars);

    // Analize the arguments and the environment variables.
    let params = Parameters::new(&args, &env_vars);

    // load leap list
    let leaps = exe::load_leaps(&params.get_leaps_path(), params.get_leaps_dt_fmt());
    let leaps = match leaps {
        Ok(leap) => leap,
        Err(e) => {
            exe::print_err(&e);
            return exe::EXIT_CODE_NG;
        }
    };

    let print_line = match params.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc TT
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tt = utc2tai(in_utc, &leaps, params.get_dt_fmt())
            .and_then(|tai| tai2tt(&tai, params.get_dt_fmt()));

        match tt {
            Err(Error::DatetimeTooLowError(_)) => {
                // 多段階で変換を行う場合、中間の日時文字列がエラーメッセージに使われている場合があるため、入力された日時文字列に置き換える。
                someone_is_err = true;
                exe::print_err(&Error::DatetimeTooLowError(in_utc.to_string()));
            }
            Err(e) => {
                someone_is_err = true;
                exe::print_err(&e)
            }
            Ok(tt) => print_line(in_utc, &tt),
        }
    }

    return if someone_is_err {
        exe::EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        exe::EXIT_CODE_OK
    };
}
