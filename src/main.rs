use clap::Clap;

mod input;
use input::Opts;

mod algorithms;
use algorithms::{md5, AlgorithmsList};

fn main() {
    let opts: Opts = Opts::parse();
    match opts.algorithm {
        AlgorithmsList::md5(options) => { 
            md5::hash_controller(&options.payload);
         },
        _ => println!("none")
    }
}