mod input;
use input::{get_opts, Opts};

mod hashers;

fn main() {
    let opts: Opts = get_opts();
    println!("{:?}", opts.algorithm);
    println!("{:?}", opts.algorithm);
}