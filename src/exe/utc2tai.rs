use super::{Arguments, EnvValues, Parameters};
use crate::{exe, utc2tai};
use std::ffi::OsString;
use std::io::Write;

pub fn main_inner(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    env_vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    stdout: &mut impl Write,
) -> i32 {
    let args = Arguments::new("Converter from UTC to TAI", args);
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

    // calc TAI
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tai = utc2tai(in_utc, &leaps, params.get_dt_fmt());

        match tai {
            Err(e) => {
                someone_is_err = true;
                exe::print_err(&e)
            }
            Ok(tai) => print_line(stdout, in_utc, &tai),
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.001\n\
            2015-07-01T00:00:06.002\n\
            2015-07-01T00:00:07.003\n\
            2015-07-01T00:00:08.004\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.000\n\
            2015-07-01T00:00:06.000\n\
            2015-07-01T00:00:07.000\n\
            2015-07-01T00:00:08.000\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.000\n\
            2015-07-01T00:00:06.000\n\
            2015-07-01T00:00:07.000\n\
            2015-07-01T00:00:08.000\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150701000004\n\
            20150701000005\n\
            20150701000006\n\
            20150701000007\n\
            20150701000008\n\
            20170101000005\n\
            20170101000006\n\
            20170101000007\n\
            20170101000008\n\
            20170101000009\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150701000004\n\
            20150701000005\n\
            20150701000006\n\
            20150701000007\n\
            20150701000008\n\
            20170101000005\n\
            20170101000006\n\
            20170101000007\n\
            20170101000008\n\
            20170101000009\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015/07/01-00:00:04\n\
            2015/07/01-00:00:05\n\
            2015/07/01-00:00:06\n\
            2015/07/01-00:00:07\n\
            2015/07/01-00:00:08\n\
            2017/01/01-00:00:05\n\
            2017/01/01-00:00:06\n\
            2017/01/01-00:00:07\n\
            2017/01/01-00:00:08\n\
            2017/01/01-00:00:09\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.000\n\
            2015-07-01T00:00:06.000\n\
            2015-07-01T00:00:07.000\n\
            2015-07-01T00:00:08.000\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.000\n\
            2015-07-01T00:00:06.000\n\
            2015-07-01T00:00:07.000\n\
            2015-07-01T00:00:08.000\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
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
        let mut stdout_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:04.000\n\
            2015-07-01T00:00:05.000\n\
            2015-07-01T00:00:06.000\n\
            2015-07-01T00:00:07.000\n\
            2015-07-01T00:00:08.000\n\
            2017-01-01T00:00:05.000\n\
            2017-01-01T00:00:06.000\n\
            2017-01-01T00:00:07.000\n\
            2017-01-01T00:00:08.000\n\
            2017-01-01T00:00:09.000\n"
        );
    }
}
