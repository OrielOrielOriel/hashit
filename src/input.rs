/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Payload
    - Text or file?

and so on.
*/
use clap::Clap;
use std::path::Path;
use crate::algorithms;

#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>")]
struct Opts {
    /// The hashing algorithm to use.
    #[clap(subcommand)]
    algorithm: algorithms::Algorithm, 

    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

pub fn check_if_file(payload: &String) -> bool {
    let path = Path::new(payload);
    path.is_file()
}

pub fn get_opts() -> () {
    let opts: Opts = Opts::parse();
    
    match opts.algorithm {
        algorithms::Algorithm::md5(options) => {
            if check_if_file(&options.payload) {
                println!("file");
            } else {
                println!("not file");
            }
        }
    }
}

// pub fn get_opts() -> () {
//     let opts: Opts = Opts::parse();
    
//     let job: algorithms::Job = algorithms::Job {
//         name = opts.algorithm,
//         data = opts.algorithm.payload,
//         id = 1234,
//     } 
// }