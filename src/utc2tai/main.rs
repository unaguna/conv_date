extern crate conv_date;
use conv_date::exe::utc2tai::main_inner;
use std::env;

fn main() {
    main_inner(env::args(), env::vars())
}
