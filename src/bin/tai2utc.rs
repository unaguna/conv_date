extern crate conv_date;
use conv_date::exe::tai2utc::main_inner;
use std::{env, io};

fn main() {
    let exit_code = main_inner(env::args(), env::vars(), &mut io::stdout());
    std::process::exit(exit_code);
}
