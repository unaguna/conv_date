//! This binary crate converts datetimes
//! from [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time)
//! to [TT](https://en.wikipedia.org/wiki/Terrestrial_Time).
//!
//! For example:
//! ```bash
//! $ utc2tt 2016-12-31T23:59:60
//! 2017-01-01T00:01:08.184
//! ```
//!
//! As you can see from the above example, it takes leap seconds into account.
//!
//! In this execution, it assume that
//! TT = [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time) + 32.184.
//!
//! # Arguments
//! It takes one or more datetimes as argument.
//! ```bash
//! $ utc2tt 2016-12-31T23:59:59 2016-12-31T23:59:60 2017-01-01T00:00:00
//! 2017-01-01T00:01:07.184
//! 2017-01-01T00:01:08.184
//! 2017-01-01T00:01:09.184
//! ```
//!
//! # Options
//! - `--dt-fmt <dt_fmt>`
//!
//!     [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of input/output datetimes. For example:
//!     ```bash
//!     $ utc2tt --dt-fmt %Y%m%d%H%M%S%.3f 20161231235960.000
//!     20170101000108.184
//!     ```
//!
//!     If both of this option and an environment variable `DT_FMT` are not specified, it uses the default: `%Y-%m-%dT%H:%M:%S%.3f`.
//!
//!  - `-H`, `--io-pair`
//!
//!     If it is specified, not only converted datetime but also input datetime are output. For example:
//!     ```bash
//!     $ utc2tt -H 2016-12-31T23:59:59 2016-12-31T23:59:60 2017-01-01T00:00:00
//!     2016-12-31T23:59:59 2017-01-01T00:01:07.184
//!     2016-12-31T23:59:60 2017-01-01T00:01:08.184
//!     2017-01-01T00:00:00 2017-01-01T00:01:09.184
//!     ```
//!
//! - `--tai-utc-table <leaps_table_file>`
//!
//!     It specifies a file which contains definition of leaps.
//!     If you use it, the option `--tai-utc-table-dt-fmt` may be useful.
//!
//!     If both of this option and an environment variable `TAI_UTC_TABLE` are not specified,
//!     it uses the default: `tai-utc.txt` in directory of executable file.
//!     If the default file also does not exist, use the built-in table in the program.
//!
//! - `--tai-utc-table-dt-fmt <tai_utc_table_dt_fmt>`
//!
//!     [format](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) of datetimes in TAI-UTC table file.
//!     It will be used when you use your TAI-UTC table file with the option `--tai-utc-table`.
//!
//!     If both of this option and an environment variable `LEAPS_DT_FMT` are not specified, it uses the default: `%Y-%m-%dT%H:%M:%S%.3f`.
//!
//! - `-V`, `--version`
//!
//!     Show the version of this executable.
//!
//! - `-h`, `--help`
//!
//!     Show help document of this executable.
//!
//! # Environment variables
//!
//! It looks for below environment variables.
//!
//! - `DT_FMT`
//!
//!     Look for a description for an option `--dt-fmt`.
//!
//! - `TAI_UTC_TABLE`
//!
//!     Look for a description for an option `--tai-utc-table`.
//!
//! - `LEAPS_DT_FMT`
//!
//!     Look for a description for an option `--tai-utc-table-dt-fmt`.
//!

extern crate convdate;
use convdate::exe::utc2tt::main_inner;
use std::{env, io};

#[doc(hidden)]
fn main() {
    let exit_code = main_inner(
        env::args(),
        env::vars(),
        &mut io::stdout(),
        &mut io::stderr(),
    );
    std::process::exit(exit_code);
}
