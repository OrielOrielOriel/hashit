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
use crate::algorithms::*;

#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>")]
struct Opts {
    /// The hashing algorithm to use.
    #[clap(subcommand)]
    algorithm: AlgorithmsList, 

    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn get_options() -> Opts {
    let ret: Opts = Opts::parse();
    ret
}

pub fn instantiate_task(options: Opts) -> TerminalTask {
    let mut task: TerminalTask = TerminalTask::new();

    task
}

fn check_if_file(payload: &String) -> bool {
    let path: &Path = Path::new(payload);
    path.is_file()
}

// pub fn get_opts() -> () {
//     let opts: Opts = Opts::parse();
    
//     let job: algorithms::Job = algorithms::Job {
//         name = opts.algorithm,
//         data = opts.algorithm.payload,
//         id = 1234,
//     } 
// }
// let opts: Opts = Opts::parse();
// match opts.algorithm {
//     AlgorithmsList::md5(options) => { 
//         md5::hash_controller(&options.payload);
//      },
//     _ => println!("none")
// }