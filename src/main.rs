use clap::Clap;

mod input;
use input::*;

mod algorithms;
use algorithms::md5;

fn main() {
    println!("{:?}", input::get_test_options())
}