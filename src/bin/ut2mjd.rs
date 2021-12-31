//! This binary crate converts datetimes
//! from [UT](https://en.wikipedia.org/wiki/Universal_Time)
//! to [MJD](https://en.wikipedia.org/wiki/Julian_day#Variants).
//!
//! For example:
//! ```bash
//! $ ut2mjd 2021-12-26T12:00:00
//! 59574.5
//! ```
//!
//! The command **doesn't** take leap seconds into account.
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

use convdate::exe::ut2mjd::main_inner;
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
