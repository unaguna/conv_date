extern crate conv_date;
use conv_date::exe::utc2tt::main_inner;
use std::env;

fn main() {
    main_inner(env::args(), env::vars())
}
