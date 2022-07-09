mod md5;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] 
struct Args {
  #[clap()]
  input: String,
}

fn main() {
  let args = Args::parse();
    
  println!("{:?}", digest);
}