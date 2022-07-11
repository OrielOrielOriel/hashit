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
        algorithms::Algo::Md4 => {
          println!("{}", algorithms::md4::hash(&args.input))
        }
        algorithms::Algo::Md2 => {
          println!("{}", algorithms::md2::hash(&args.input))
        }
        algorithms::Algo::Sha1 => {
            println!("{}", algorithms::sha1::hash(&args.input))
        }
        algorithms::Algo::Sha224 => {
          println!("{}", algorithms::sha224::hash(&args.input))
        }
        algorithms::Algo::Sha256 => {
          println!("{}", algorithms::sha256::hash(&args.input))
        }
        algorithms::Algo::Sha384 => {
          println!("{}", algorithms::sha384::hash(&args.input))
        }
        algorithms::Algo::Sha512 => {
          println!("{}", algorithms::sha512::hash(&args.input))
        }
        algorithms::Algo::Sha512_224 => {
          println!("{}", algorithms::sha512_224::hash(&args.input))
        }
        algorithms::Algo::Sha512_256 => {
          println!("{}", algorithms::sha512_256::hash(&args.input))
        }
        algorithms::Algo::Ntlm => {
          println!("{}", algorithms::ntlm::hash(&args.input))
        }
    }
}
