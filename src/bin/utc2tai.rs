extern crate convdate;
use convdate::exe::utc2tai::main_inner;
use std::{env, io};

fn main() {
    let exit_code = main_inner(
        env::args(),
        env::vars(),
        &mut io::stdout(),
        &mut io::stderr(),
    );
    std::process::exit(exit_code);
}
