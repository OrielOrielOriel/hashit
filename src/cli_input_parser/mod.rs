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

#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>")]
pub struct Opts {
    /// The hashing algorithm to use.
    #[clap(subcommand)]
    pub algorithm: Algorithm, 

    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

#[derive(Clap)]
pub enum Algorithm {
    MD5(MD5),
    MD2(MD2)
}

/// The MD5 Hashing algorithm
#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "The MD5 Hashing algorithm boop")]
pub struct MD5 {
    /// A text string or file.
    pub payload: String,
}

/// The MD2 Hashing algorithm
#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "The MD2 Hashing algorithm boop")]
pub struct MD2 {
    /// A text string or file.
    pub payload: String,
}

pub fn get_opts() -> Opts {
    let opts: Opts = Opts::parse();
    opts
}