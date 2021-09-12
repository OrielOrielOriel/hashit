use clap::Clap;

mod input;
use input::*;

mod algorithms;
use algorithms::md5;

fn main() {
    let opts = get_options();
    match opts.algorithm {
        AlgorithmsOptsList::md5(options) => {
            
        }
    }
}