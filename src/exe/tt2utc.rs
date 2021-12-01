use super::{Arguments, EnvValues, Parameters};
use crate::{error::Error, exe, tai2utc, tt2tai};
use std::ffi::OsString;
use std::io::Write;

pub fn main_inner(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    env_vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    stdout: &mut impl Write,
) -> i32 {
    let args = Arguments::new("Converter from TT to UTC", args);
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

    let print_line: fn(&mut dyn Write, &str, &str) -> () = match params.io_pair_flg() {
        false => |out: &mut dyn Write, _: &str, o: &str| writeln!(out, "{}", o).unwrap(),
        true => |out: &mut dyn Write, i: &str, o: &str| writeln!(out, "{} {}", i, o).unwrap(),
    };

    // calc UTC
    let mut someone_is_err = false;
    for in_tt in args.get_datetimes() {
        let utc = tt2tai(in_tt, params.get_dt_fmt())
            .and_then(|tai| tai2utc(&tai, &leaps, params.get_dt_fmt()));

        match utc {
            Err(Error::DatetimeTooLowError(_)) => {
                // 多段階で変換を行う場合、中間の日時文字列がエラーメッセージに使われている場合があるため、入力された日時文字列に置き換える。
                someone_is_err = true;
                exe::print_err(&Error::DatetimeTooLowError(in_tt.to_string()));
            }
            Err(e) => {
                someone_is_err = true;
                exe::print_err(&e)
            }
            Ok(utc) => print_line(stdout, in_tt, &utc),
        }
    }

    return if someone_is_err {
        exe::EXIT_CODE_SOME_DT_NOT_CONVERTED
    } else {
        exe::EXIT_CODE_OK
    };
}

#[cfg(test)]
mod tests {
    use super::super::testmod;
    use super::main_inner;
    use std::collections::HashMap;
    use std::io;

    const EXE_NAME: &str = "utc2tt";

    /// Test regular case.
    ///
    /// It runs with no arguments and just one environment value "LEAPS_TABLE" for reproducibility
    /// but originally the minimum execution is with no environment values.
    /// If "LEAPS_TABLE" is not specified, the program uses file in the directory of the executable file.
    #[test]
    fn test_simply() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59.000",
            "2015-06-30T23:59:60.001",
            "2015-07-01T00:00:00.002",
            "2015-07-01T00:00:01.003",
            "2015-07-01T00:00:02.004",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
        ];
        let env_vars = HashMap::from([("LEAPS_TABLE", leaps_table_path.to_str().unwrap())]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test an argunent --leaps-table.
    #[test]
    fn test_arg_leaps_table() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59",
            "2015-06-30T23:59:60",
            "2015-07-01T00:00:00",
            "2015-07-01T00:00:01",
            "2015-07-01T00:00:02",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
            "--leaps-table",
            leaps_table_path.to_str().unwrap(),
        ];
        let env_vars: HashMap<&str, &str> = HashMap::from([]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test that an argument --leaps-table has a priority to an environment variable LEAPS_TABLE.
    #[test]
    fn test_arg_leaps_table_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();
        let dummy_leaps_table_path =
            testmod::tmp_text_file(&test_dir, "dummy_leap.txt", &vec!["XXX"]).unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59",
            "2015-06-30T23:59:60",
            "2015-07-01T00:00:00",
            "2015-07-01T00:00:01",
            "2015-07-01T00:00:02",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
            "--leaps-table",
            leaps_table_path.to_str().unwrap(),
        ];
        let env_vars = HashMap::from([("LEAPS_TABLE", dummy_leaps_table_path.to_str().unwrap())]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test an argunent --dt-fmt.
    #[test]
    fn test_arg_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "20150630235959",
            "20150630235960",
            "20150701000000",
            "20150701000001",
            "20150701000002",
            "20161231235959",
            "20161231235960",
            "20170101000000",
            "20170101000001",
            "20170101000002",
            "--dt-fmt",
            "%Y%m%d%H%M%S",
        ];
        let env_vars = HashMap::from([("LEAPS_TABLE", leaps_table_path.to_str().unwrap())]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test an environment variable DT_FMT.
    #[test]
    fn test_env_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "20150630235959",
            "20150630235960",
            "20150701000000",
            "20150701000001",
            "20150701000002",
            "20161231235959",
            "20161231235960",
            "20170101000000",
            "20170101000001",
            "20170101000002",
        ];
        let env_vars = HashMap::from([
            ("LEAPS_TABLE", leaps_table_path.to_str().unwrap()),
            ("DT_FMT", "%Y%m%d%H%M%S"),
        ]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test that an argunent --dt-fmt has a priority to an environment variable DT_FMT.
    #[test]
    fn test_arg_dt_fmt_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015/06/30-23:59:59",
            "2015/06/30-23:59:60",
            "2015/07/01-00:00:00",
            "2015/07/01-00:00:01",
            "2015/07/01-00:00:02",
            "2016/12/31-23:59:59",
            "2016/12/31-23:59:60",
            "2017/01/01-00:00:00",
            "2017/01/01-00:00:01",
            "2017/01/01-00:00:02",
            "--dt-fmt",
            "%Y/%m/%d-%H:%M:%S",
        ];
        let env_vars = HashMap::from([
            ("LEAPS_TABLE", leaps_table_path.to_str().unwrap()),
            ("DT_FMT", "%Y%m%d%H%M%S"),
        ]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test an argunent --leaps-dt-fmt.
    #[test]
    fn test_arg_leaps_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "20120701000000000 5",
                "20150701000000000 6",
                "20170101000000000 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59",
            "2015-06-30T23:59:60",
            "2015-07-01T00:00:00",
            "2015-07-01T00:00:01",
            "2015-07-01T00:00:02",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
            "--leaps-dt-fmt",
            "%Y%m%d%H%M%S%3f",
        ];
        let env_vars = HashMap::from([("LEAPS_TABLE", leaps_table_path.to_str().unwrap())]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test an environment variable LEAPS_DT_FMT.
    #[test]
    fn test_env_leaps_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "20120701000000000 5",
                "20150701000000000 6",
                "20170101000000000 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59",
            "2015-06-30T23:59:60",
            "2015-07-01T00:00:00",
            "2015-07-01T00:00:01",
            "2015-07-01T00:00:02",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
        ];
        let env_vars = HashMap::from([
            ("LEAPS_TABLE", leaps_table_path.to_str().unwrap()),
            ("LEAPS_DT_FMT", "%Y%m%d%H%M%S%3f"),
        ]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }

    /// Test that an argunent --leaps-dt-fmt has a priority to an environment variable LEAPS_DT_FMT
    #[test]
    fn test_arg_leaps_dt_fmt_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
            &test_dir,
            &vec![
                "2012/07/01-00:00:00 5",
                "2015/07/01-00:00:00 6",
                "2017/01/01-00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-06-30T23:59:59",
            "2015-06-30T23:59:60",
            "2015-07-01T00:00:00",
            "2015-07-01T00:00:01",
            "2015-07-01T00:00:02",
            "2016-12-31T23:59:59",
            "2016-12-31T23:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
            "--leaps-dt-fmt",
            "%Y/%m/%d-%H:%M:%S",
        ];
        let env_vars = HashMap::from([
            ("LEAPS_TABLE", leaps_table_path.to_str().unwrap()),
            ("LEAPS_DT_FMT", "%Y%m%d%H%M%S%3f"),
        ]);

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut io::stdout());

        assert_eq!(exec_code, 0);
    }
}
