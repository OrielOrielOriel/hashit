use clap::Clap;

mod input;
use input::*;

mod algorithms;
use algorithms::md5;

fn main() {
    let app = input::iniate_clap_app();
    input::generate_task(app);
}