mod algorithms;
use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] 
struct Args {
  #[clap(arg_enum)]
  algorithm: Algo,

  #[clap()]
  input: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algo {
  md5,
}

fn main() {
  let args = Args::parse();

  match args.algorithm {
    Algo::md5 => { println!("{:?}", algorithms::md5::hash(args.input)) },
    
  }
}