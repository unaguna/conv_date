extern crate conv_date;
use conv_date::exe::tt2utc::main_inner;
use std::env;

fn main() {
    let exit_code = main_inner(env::args(), env::vars());
    std::process::exit(exit_code);
}
