//! This binary crate converts datetimes
//! from [TAI](https://en.wikipedia.org/wiki/International_Atomic_Time)
//! to [UTC](https://en.wikipedia.org/wiki/Coordinated_Universal_Time).
//!
//! For example:
//! ```bash
//! $ tai2utc 2017-01-01T00:00:36
//! 2016-12-31T23:59:60.000
//! ```
//!
//! As you can see from the above example, it takes leap seconds into account.
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

use convdate::exe::tai2utc::main_inner;
use std::env;
use std::io;

#[doc(hidden)]
fn main() {
    let exit_code = main_inner(
        env::args(),
        env::vars(),
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
        &mut io::stderr(),
    );
    std::process::exit(exit_code);
}
