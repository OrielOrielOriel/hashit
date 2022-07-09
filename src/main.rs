use md5;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] 
struct Args {
  #[clap()]
  input: String,
}

fn main() {
  let args = Args::parse();
  
  let digest = md5::compute(&args.input);
  println!("{:?}", digest);
}