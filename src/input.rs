/*
When the user provides CLI input, we need to parse it for:
- Hashing algorithm
- Verbose mode to level
- Payload
    - Text or file?

and so on.
*/
use clap::{Clap, ArgMatches};
use std::path::Path;
use crate::algorithms::*;

trait Opts {
    fn new() -> Self;
    fn generate(&self) -> Self;
}

struct Options {
    arguments: ArgMatches,
    payload: Option<String>,
    hashing_algorithm: Option<String>,
    verbosity: Option<u8>,
}

impl Opts for Options {

}

#[derive(Clap, Debug)]
pub struct GenericAlgorithmOpts {
    #[clap(
        value_delimiter = ' ',
        setting = clap::ArgSettings::UseValueDelimiter,
        setting = clap::ArgSettings::MultipleValues,
        setting = clap::ArgSettings::TakesValue,
    )]
    pub payload: String
}

#[derive(Clap, Debug)]
#[allow(non_camel_case_types)]
pub enum AlgorithmsOptsList {
    md5(GenericAlgorithmOpts),
}

// pub fn instantiate_task(options: Opts) -> TerminalTask {
//     let mut ret: TerminalTask = TerminalTask::new();

//     ret
// }

