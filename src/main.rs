use clap::Clap;

mod input;
use input::Opts;

mod algorithms;
use algorithms::{md5, Algorithms_List};

fn main() {
    let opts: Opts = Opts::parse();
    match opts.algorithm {
        Algorithms_List::md5(options) => { 
            md5::hash_controller(&options.payload);
         },
        _ => println!("none")
    }
}