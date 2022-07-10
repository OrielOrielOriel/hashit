mod algorithms;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(arg_enum)]
    algorithm: algorithms::Algo,

    #[clap()]
    input: String,
}

fn main() {
    let args = Args::parse();

    match args.algorithm {
        algorithms::Algo::Md5 => {
            println!("{}", algorithms::md5::hash(&args.input))
        }
        algorithms::Algo::Sha1 => {
            println!("{}", algorithms::sha1::hash(&args.input))
        }
    }
}
