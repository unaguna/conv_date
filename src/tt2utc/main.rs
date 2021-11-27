extern crate conv_date;
use conv_date::exe::tt2utc::main_inner;
use std::env;

fn main() {
    main_inner(env::args(), env::vars())
}
