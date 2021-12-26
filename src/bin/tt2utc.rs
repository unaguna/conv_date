//! This binary crate converts datetimes
//! from [TT](https://en.wikipedia.org/wiki/Terrestrial_Time)
//! to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
//!
//! For example:
//! ```bash
//! $ tt2utc 2017-01-01T00:01:09
//! 2016-12-31T23:59:60.816
//! ```
//!
//! As you can see from the above example, it takes leap seconds into account.
//!
//! In this execution, it assume that
//! TT = [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time) + 32.184.
//!
//! # Arguments
//! See [utc2tt#Arguments](../utc2tt/index.html#arguments).
//!
//! # Options
//! See [utc2tt#Options](../utc2tt/index.html#options).
//!
//! # Environment variables
//! See [utc2tt#Environment variables](../utc2tt/index.html#environment-variables).
//!
//! # Standard input
//! See [utc2tt#Standard input](../utc2tt/index.html#standard-input).

use convdate::exe::tt2utc::main_inner;
use std::env;
use std::io::{self, BufWriter};

#[doc(hidden)]
fn main() {
    let exit_code = main_inner(
        env::args(),
        env::vars(),
        &mut io::stdin().lock(),
        &mut BufWriter::new(io::stdout().lock()),
        &mut io::stderr(),
    );
    std::process::exit(exit_code);
}
