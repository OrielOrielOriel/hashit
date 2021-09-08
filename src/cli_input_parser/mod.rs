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

    /// The payload, can be either a text string or a filepath.
    #[clap(short, long)]
    pub payload: String,

    /// The hashing algorithm, a text string.
    #[clap(short, long)]
    pub algorithm: String,

    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

pub fn get_opts() -> (Opts, bool) {
    let opts: Opts = Opts::parse();
    let is_payload_file: bool = Path::new(&opts.payload).is_file();

    (opts, is_payload_file)
}