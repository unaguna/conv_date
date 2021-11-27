use conv_date::{error::Error, exe, tai2tt, utc2tai};

fn main() {
    // Analize the arguments
    let args = exe::Arguments::new("Converter from UTC to TT");

    // load leap list
    let leaps = args
        .get_leaps_path()
        .and_then(|p| exe::load_leaps(&p, args.get_leaps_dt_fmt()))
        .unwrap_or_else(|e| {
            exe::print_err(&e);
            std::process::exit(exe::EXIT_CODE_NG)
        });

    let print_line = match args.io_pair_flg() {
        false => |_: &str, o: &str| println!("{}", o),
        true => |i: &str, o: &str| println!("{} {}", i, o),
    };

    // calc TT
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tt = utc2tai(in_utc, &leaps, args.get_dt_fmt())
            .and_then(|tai| tai2tt(&tai, args.get_dt_fmt()));

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

    std::process::exit(if someone_is_err {
        exe::EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        exe::EXIT_CODE_OK
    });
}
