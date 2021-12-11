use super::{Arguments, EnvValues, Parameters};
use crate::{exe, tt2utc};
use std::ffi::OsString;
use std::io::Write;

pub fn main_inner(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    env_vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> i32 {
    let args = Arguments::new("Converter from TT to UTC", args);
    let env_vars = EnvValues::new(env_vars);

    // Analyze the arguments and the environment variables.
    let params = Parameters::new(&args, &env_vars);

    // load TAI-UTC table
    let tai_utc_table = exe::load_tai_utc_table(
        params.get_tai_utc_table_path(),
        params.get_tai_utc_table_dt_fmt(),
    );
    let tai_utc_table = match tai_utc_table {
        Ok(tai_utc_table) => tai_utc_table,
        Err(e) => {
            exe::print_err(stderr, &e);
            return exe::EXIT_CODE_NG;
        }
    };

    // function for output to stdout
    let print_line = exe::get_print_line(&params);

    // calc UTC
    let mut someone_is_err = false;
    for in_tt in args.get_datetimes() {
        let utc = tt2utc(in_tt, &tai_utc_table, params.get_dt_fmt());

        match utc {
            Err(e) => {
                someone_is_err = true;
                exe::print_err(stderr, &e)
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
    use crate::exe;
    use std::collections::HashMap;

    const EXE_NAME: &str = "utc2tt";

    /// Test regular case.
    #[test]
    fn test_simply() {
        let args = vec![
            EXE_NAME,
            "2015-07-01T00:01:06.184",
            "2015-07-01T00:01:07.185",
            "2015-07-01T00:01:08.186",
            "2015-07-01T00:01:09.187",
            "2015-07-01T00:01:10.188",
            "2017-01-01T00:01:07.184",
            "2017-01-01T00:01:08.184",
            "2017-01-01T00:01:09.184",
            "2017-01-01T00:01:10.184",
            "2017-01-01T00:01:11.184",
        ];
        let env_vars = HashMap::<String, String>::from([]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:59.000\n\
            2015-06-30T23:59:60.001\n\
            2015-07-01T00:00:00.002\n\
            2015-07-01T00:00:01.003\n\
            2015-07-01T00:00:02.004\n\
            2016-12-31T23:59:59.000\n\
            2016-12-31T23:59:60.000\n\
            2017-01-01T00:00:00.000\n\
            2017-01-01T00:00:01.000\n\
            2017-01-01T00:00:02.000\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test error when input datetimes are illegal.
    #[test]
    fn test_input_dt_illegal_against_default_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36.184",
            "2015-07-01T00:00:37.185",
            "2010-07-01T00:00:38.186",
            "2015-07-01T00:00:39.187",
            "2015-07-01T00:00:40.188",
            "2017-01-0100:00:37.184",
            "2017-01-0100:00:38.184",
            "2017-01-01T00:00:39.184",
            "2017-01-01T00:00:40.184",
            "2017-01-01T00:00:41.184",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 2);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:59.000\n\
            2015-06-30T23:59:60.001\n\
            2015-07-01T00:00:01.003\n\
            2015-07-01T00:00:02.004\n\
            2017-01-01T00:00:00.000\n\
            2017-01-01T00:00:01.000\n\
            2017-01-01T00:00:02.000\n"
        );
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n{}: {}\n{}: {}\n",
                exe::exe_name(),
                "The datetime is too low: 2010-07-01 00:00:38.186",
                exe::exe_name(),
                "Cannot parse the datetime: 2017-01-0100:00:37.184",
                exe::exe_name(),
                "Cannot parse the datetime: 2017-01-0100:00:38.184"
            )
        );
    }

    /// Test error when leaps data are illegal.
    #[test]
    fn test_leaps_illegal() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-01T00:00:00 A",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-07-01T00:00:36.184",
            "2015-07-01T00:00:37.185",
            "2015-07-01T00:00:38.186",
            "2015-07-01T00:00:39.187",
            "2015-07-01T00:00:40.188",
            "2017-01-01T00:00:37.184",
            "2017-01-01T00:00:38.184",
            "2017-01-01T00:00:39.184",
            "2017-01-01T00:00:40.184",
            "2017-01-01T00:00:41.184",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 1);
        assert_eq!(String::from_utf8_lossy(&stdout_buf), "");
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n",
                exe::exe_name(),
                "Illegal definition of TAI-UTC difference: 2015-07-01T00:00:00 A"
            )
        );
    }

    /// Test error when leaps datetimes are illegal.
    #[test]
    fn test_leaps_dt_illegal_against_default_tai_utc_table_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
            &test_dir,
            &vec![
                "2012-07-01T00:00:00 5",
                "2015-07-0100:00:00 6",
                "2017-01-01T00:00:00 7",
            ],
        )
        .unwrap();

        let args = vec![
            EXE_NAME,
            "2015-07-01T00:00:36.184",
            "2015-07-01T00:00:37.185",
            "2015-07-01T00:00:38.186",
            "2015-07-01T00:00:39.187",
            "2015-07-01T00:00:40.188",
            "2017-01-01T00:00:37.184",
            "2017-01-01T00:00:38.184",
            "2017-01-01T00:00:39.184",
            "2017-01-01T00:00:40.184",
            "2017-01-01T00:00:41.184",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 1);
        assert_eq!(String::from_utf8_lossy(&stdout_buf), "");
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n",
                exe::exe_name(),
                "Illegal definition of TAI-UTC difference (datetime): 2015-07-0100:00:00"
            )
        );
    }

    /// Test error when an environment variable TAI_UTC_TABLE is a path which is not exists
    #[test]
    fn test_env_leaps_table_not_exist() {
        let leaps_table_path = "/tmp/dummy/not_exists.txt";

        let args = vec![
            EXE_NAME,
            "2015-07-01T00:00:36.184",
            "2015-07-01T00:00:37.185",
            "2015-07-01T00:00:38.186",
            "2015-07-01T00:00:39.187",
            "2015-07-01T00:00:40.188",
            "2017-01-01T00:00:37.184",
            "2017-01-01T00:00:38.184",
            "2017-01-01T00:00:39.184",
            "2017-01-01T00:00:40.184",
            "2017-01-01T00:00:41.184",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path)]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 1);
        assert_eq!(String::from_utf8_lossy(&stdout_buf), "");
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n",
                exe::exe_name(),
                "The TAI-UTC table file isn't available: /tmp/dummy/not_exists.txt"
            )
        );
    }

    /// Test an argument --tai-utc-table.
    #[test]
    fn test_arg_leaps_table() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
            "--tai-utc-table",
            leaps_table_path.to_str().unwrap(),
        ];
        let env_vars: HashMap<&str, &str> = HashMap::from([]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:58.816\n\
            2015-06-30T23:59:59.816\n\
            2015-06-30T23:59:60.816\n\
            2015-07-01T00:00:00.816\n\
            2015-07-01T00:00:01.816\n\
            2016-12-31T23:59:58.816\n\
            2016-12-31T23:59:59.816\n\
            2016-12-31T23:59:60.816\n\
            2017-01-01T00:00:00.816\n\
            2017-01-01T00:00:01.816\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test error when an argument --tai-utc-table is a path which is not exists
    #[test]
    fn test_arg_leaps_table_not_exist() {
        let leaps_table_path = "/tmp/dummy/not_exists.txt";

        let args = vec![
            EXE_NAME,
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
            "--tai-utc-table",
            leaps_table_path,
        ];
        let env_vars: HashMap<&str, &str> = HashMap::from([]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 1);
        assert_eq!(String::from_utf8_lossy(&stdout_buf), "");
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n",
                exe::exe_name(),
                "The TAI-UTC table file isn't available: /tmp/dummy/not_exists.txt"
            )
        );
    }

    /// Test an environment variable TAI_UTC_TABLE.
    #[test]
    fn test_env_leaps_table() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36.184",
            "2015-07-01T00:00:37.185",
            "2015-07-01T00:00:38.186",
            "2015-07-01T00:00:39.187",
            "2015-07-01T00:00:40.188",
            "2017-01-01T00:00:37.184",
            "2017-01-01T00:00:38.184",
            "2017-01-01T00:00:39.184",
            "2017-01-01T00:00:40.184",
            "2017-01-01T00:00:41.184",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:59.000\n\
            2015-06-30T23:59:60.001\n\
            2015-07-01T00:00:00.002\n\
            2015-07-01T00:00:01.003\n\
            2015-07-01T00:00:02.004\n\
            2016-12-31T23:59:59.000\n\
            2016-12-31T23:59:60.000\n\
            2017-01-01T00:00:00.000\n\
            2017-01-01T00:00:01.000\n\
            2017-01-01T00:00:02.000\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test that an argument --tai-utc-table has a priority to an environment variable TAI_UTC_TABLE.
    #[test]
    fn test_arg_leaps_table_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
            "--tai-utc-table",
            leaps_table_path.to_str().unwrap(),
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", dummy_leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:58.816\n\
            2015-06-30T23:59:59.816\n\
            2015-06-30T23:59:60.816\n\
            2015-07-01T00:00:00.816\n\
            2015-07-01T00:00:01.816\n\
            2016-12-31T23:59:58.816\n\
            2016-12-31T23:59:59.816\n\
            2016-12-31T23:59:60.816\n\
            2017-01-01T00:00:00.816\n\
            2017-01-01T00:00:01.816\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an argument --dt-fmt.
    #[test]
    fn test_arg_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "20150701000036",
            "20150701000037",
            "20150701000038",
            "20150701000039",
            "20150701000040",
            "20170101000037",
            "20170101000038",
            "20170101000039",
            "20170101000040",
            "20170101000041",
            "--dt-fmt",
            "%Y%m%d%H%M%S",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150630235958\n\
            20150630235959\n\
            20150630235960\n\
            20150701000000\n\
            20150701000001\n\
            20161231235958\n\
            20161231235959\n\
            20161231235960\n\
            20170101000000\n\
            20170101000001\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an environment variable DT_FMT.
    #[test]
    fn test_env_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "20150701000036",
            "20150701000037",
            "20150701000038",
            "20150701000039",
            "20150701000040",
            "20170101000037",
            "20170101000038",
            "20170101000039",
            "20170101000040",
            "20170101000041",
        ];
        let env_vars = HashMap::from([
            ("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap()),
            ("DT_FMT", "%Y%m%d%H%M%S"),
        ]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150630235958\n\
            20150630235959\n\
            20150630235960\n\
            20150701000000\n\
            20150701000001\n\
            20161231235958\n\
            20161231235959\n\
            20161231235960\n\
            20170101000000\n\
            20170101000001\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test that an argument --dt-fmt has a priority to an environment variable DT_FMT.
    #[test]
    fn test_arg_dt_fmt_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015/07/01-00:00:36",
            "2015/07/01-00:00:37",
            "2015/07/01-00:00:38",
            "2015/07/01-00:00:39",
            "2015/07/01-00:00:40",
            "2017/01/01-00:00:37",
            "2017/01/01-00:00:38",
            "2017/01/01-00:00:39",
            "2017/01/01-00:00:40",
            "2017/01/01-00:00:41",
            "--dt-fmt",
            "%Y/%m/%d-%H:%M:%S",
        ];
        let env_vars = HashMap::from([
            ("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap()),
            ("DT_FMT", "%Y%m%d%H%M%S"),
        ]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015/06/30-23:59:58\n\
            2015/06/30-23:59:59\n\
            2015/06/30-23:59:60\n\
            2015/07/01-00:00:00\n\
            2015/07/01-00:00:01\n\
            2016/12/31-23:59:58\n\
            2016/12/31-23:59:59\n\
            2016/12/31-23:59:60\n\
            2017/01/01-00:00:00\n\
            2017/01/01-00:00:01\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an argument --tai-utc-table-dt-fmt.
    #[test]
    fn test_arg_tai_utc_table_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
            "--tai-utc-table-dt-fmt",
            "%Y%m%d%H%M%S%3f",
        ];
        let env_vars = HashMap::from([("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:58.816\n\
            2015-06-30T23:59:59.816\n\
            2015-06-30T23:59:60.816\n\
            2015-07-01T00:00:00.816\n\
            2015-07-01T00:00:01.816\n\
            2016-12-31T23:59:58.816\n\
            2016-12-31T23:59:59.816\n\
            2016-12-31T23:59:60.816\n\
            2017-01-01T00:00:00.816\n\
            2017-01-01T00:00:01.816\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an environment variable TAI_UTC_TABLE_DT_FMT.
    #[test]
    fn test_env_tai_utc_table_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
        ];
        let env_vars = HashMap::from([
            ("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap()),
            ("TAI_UTC_TABLE_DT_FMT", "%Y%m%d%H%M%S%3f"),
        ]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:58.816\n\
            2015-06-30T23:59:59.816\n\
            2015-06-30T23:59:60.816\n\
            2015-07-01T00:00:00.816\n\
            2015-07-01T00:00:01.816\n\
            2016-12-31T23:59:58.816\n\
            2016-12-31T23:59:59.816\n\
            2016-12-31T23:59:60.816\n\
            2017-01-01T00:00:00.816\n\
            2017-01-01T00:00:01.816\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test that an argument --tai-utc-table-dt-fmt has a priority to an environment variable TAI_UTC_TABLE_DT_FMT
    #[test]
    fn test_arg_tai_utc_table_dt_fmt_against_env() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_tai_utc_table(
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
            "2015-07-01T00:00:36",
            "2015-07-01T00:00:37",
            "2015-07-01T00:00:38",
            "2015-07-01T00:00:39",
            "2015-07-01T00:00:40",
            "2017-01-01T00:00:37",
            "2017-01-01T00:00:38",
            "2017-01-01T00:00:39",
            "2017-01-01T00:00:40",
            "2017-01-01T00:00:41",
            "--tai-utc-table-dt-fmt",
            "%Y/%m/%d-%H:%M:%S",
        ];
        let env_vars = HashMap::from([
            ("TAI_UTC_TABLE", leaps_table_path.to_str().unwrap()),
            ("TAI_UTC_TABLE_DT_FMT", "%Y%m%d%H%M%S%3f"),
        ]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-06-30T23:59:58.816\n\
            2015-06-30T23:59:59.816\n\
            2015-06-30T23:59:60.816\n\
            2015-07-01T00:00:00.816\n\
            2015-07-01T00:00:01.816\n\
            2016-12-31T23:59:58.816\n\
            2016-12-31T23:59:59.816\n\
            2016-12-31T23:59:60.816\n\
            2017-01-01T00:00:00.816\n\
            2017-01-01T00:00:01.816\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }
}
