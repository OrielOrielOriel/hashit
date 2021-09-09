/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Payload
    - Text or file?

and so on.
*/
use clap::Clap;

use crate::hashers::{subcommand_structures};

#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>")]
pub struct Opts {
    /// The hashing algorithm to use.
    #[clap(subcommand)]
    pub algorithm: subcommand_structures::algorithm, 

    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

pub fn get_opts() -> Opts {
    let opts: Opts = Opts::parse();
    opts
}