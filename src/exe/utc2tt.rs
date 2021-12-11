use super::{Arguments, EnvValues, Parameters};
use crate::{exe, utc2tt};
use std::ffi::OsString;
use std::io::Write;

pub fn main_inner(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    env_vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> i32 {
    let args = Arguments::new("Converter from UTC to TT", args);
    let env_vars = EnvValues::new(env_vars);

    // Analyze the arguments and the environment variables.
    let params = Parameters::new(&args, &env_vars);

    // load leap list
    let leaps = exe::load_leaps(params.get_leaps_path(), params.get_leaps_dt_fmt());
    let leaps = match leaps {
        Ok(leap) => leap,
        Err(e) => {
            exe::print_err(stderr, &e);
            return exe::EXIT_CODE_NG;
        }
    };

    // function for output to stdout
    let print_line = exe::get_print_line(&params);

    // calc TT
    let mut someone_is_err = false;
    for in_utc in args.get_datetimes() {
        let tt = utc2tt(in_utc, &leaps, params.get_dt_fmt());

        match tt {
            Err(e) => {
                someone_is_err = true;
                exe::print_err(stderr, &e)
            }
            Ok(tt) => print_line(stdout, in_utc, &tt),
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
        let env_vars = HashMap::<String, String>::from([]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:01:06.184\n\
            2015-07-01T00:01:07.185\n\
            2015-07-01T00:01:08.186\n\
            2015-07-01T00:01:09.187\n\
            2015-07-01T00:01:10.188\n\
            2017-01-01T00:01:07.184\n\
            2017-01-01T00:01:08.184\n\
            2017-01-01T00:01:09.184\n\
            2017-01-01T00:01:10.184\n\
            2017-01-01T00:01:11.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test error when input datetimes are illegal.
    #[test]
    fn test_input_dt_illegal_against_default_dt_fmt() {
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
            "2010-07-01T00:00:00.002",
            "2015-07-01T00:00:01.003",
            "2015-07-01T00:00:02.004",
            "2016-12-3123:59:59",
            "2016-12-3123:59:60",
            "2017-01-01T00:00:00",
            "2017-01-01T00:00:01",
            "2017-01-01T00:00:02",
        ];
        let env_vars = HashMap::from([("LEAPS_TABLE", leaps_table_path.to_str().unwrap())]);
        let mut stdout_buf = Vec::<u8>::new();
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 2);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.185\n\
            2015-07-01T00:00:39.187\n\
            2015-07-01T00:00:40.188\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(
            String::from_utf8_lossy(&stderr_buf),
            format!(
                "{}: {}\n{}: {}\n{}: {}\n",
                exe::exe_name(),
                "The datetime is too low: 2010-07-01 00:00:00.002",
                exe::exe_name(),
                "Cannot parse the datetime: 2016-12-3123:59:59",
                exe::exe_name(),
                "Cannot parse the datetime: 2016-12-3123:59:60"
            )
        );
    }

    /// Test error when leaps data are illegal.
    #[test]
    fn test_leaps_illegal() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
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
                "Illegal leap definition: 2015-07-01T00:00:00 A"
            )
        );
    }

    /// Test error when leaps datetimes are illegal.
    #[test]
    fn test_leaps_dt_illegal_against_default_leaps_dt_fmt() {
        let test_dir = testmod::tmp_dir(Some("")).unwrap();
        let leaps_table_path = testmod::tmp_leaps_table(
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
                "Illegal leap definition (datetime): 2015-07-0100:00:00"
            )
        );
    }

    /// Test error when an environment variable LEAPS_TABLE is a path which is not exists
    #[test]
    fn test_env_leaps_table_not_exist() {
        let leaps_table_path = "/tmp/dummy/not_exists.txt";

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
        let env_vars = HashMap::from([("LEAPS_TABLE", leaps_table_path)]);
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
                "The leaps table file isn't available: /tmp/dummy/not_exists.txt"
            )
        );
    }

    /// Test an argument --leaps-table.
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.184\n\
            2015-07-01T00:00:38.184\n\
            2015-07-01T00:00:39.184\n\
            2015-07-01T00:00:40.184\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test error when an argument --leaps-table is a path which is not exists
    #[test]
    fn test_arg_leaps_table_not_exist() {
        let leaps_table_path = "/tmp/dummy/not_exists.txt";

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
                "The leaps table file isn't available: /tmp/dummy/not_exists.txt"
            )
        );
    }

    /// Test an environment variable LEAPS_TABLE.
    #[test]
    fn test_env_leaps_table() {
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.185\n\
            2015-07-01T00:00:38.186\n\
            2015-07-01T00:00:39.187\n\
            2015-07-01T00:00:40.188\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.184\n\
            2015-07-01T00:00:38.184\n\
            2015-07-01T00:00:39.184\n\
            2015-07-01T00:00:40.184\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an argument --dt-fmt.
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150701000036\n\
            20150701000037\n\
            20150701000038\n\
            20150701000039\n\
            20150701000040\n\
            20170101000037\n\
            20170101000038\n\
            20170101000039\n\
            20170101000040\n\
            20170101000041\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "20150701000036\n\
            20150701000037\n\
            20150701000038\n\
            20150701000039\n\
            20150701000040\n\
            20170101000037\n\
            20170101000038\n\
            20170101000039\n\
            20170101000040\n\
            20170101000041\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test that an argument --dt-fmt has a priority to an environment variable DT_FMT.
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015/07/01-00:00:36\n\
            2015/07/01-00:00:37\n\
            2015/07/01-00:00:38\n\
            2015/07/01-00:00:39\n\
            2015/07/01-00:00:40\n\
            2017/01/01-00:00:37\n\
            2017/01/01-00:00:38\n\
            2017/01/01-00:00:39\n\
            2017/01/01-00:00:40\n\
            2017/01/01-00:00:41\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test an argument --leaps-dt-fmt.
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.184\n\
            2015-07-01T00:00:38.184\n\
            2015-07-01T00:00:39.184\n\
            2015-07-01T00:00:40.184\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.184\n\
            2015-07-01T00:00:38.184\n\
            2015-07-01T00:00:39.184\n\
            2015-07-01T00:00:40.184\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }

    /// Test that an argument --leaps-dt-fmt has a priority to an environment variable LEAPS_DT_FMT
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
        let mut stderr_buf = Vec::<u8>::new();

        // Run the target.
        let exec_code = main_inner(args, env_vars, &mut stdout_buf, &mut stderr_buf);

        assert_eq!(exec_code, 0);
        assert_eq!(
            String::from_utf8_lossy(&stdout_buf),
            "2015-07-01T00:00:36.184\n\
            2015-07-01T00:00:37.184\n\
            2015-07-01T00:00:38.184\n\
            2015-07-01T00:00:39.184\n\
            2015-07-01T00:00:40.184\n\
            2017-01-01T00:00:37.184\n\
            2017-01-01T00:00:38.184\n\
            2017-01-01T00:00:39.184\n\
            2017-01-01T00:00:40.184\n\
            2017-01-01T00:00:41.184\n"
        );
        assert_eq!(String::from_utf8_lossy(&stderr_buf), "");
    }
}
