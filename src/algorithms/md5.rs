mod algorithms::HashingAlgorithm;
use clap::Clap;
use md5;

#[derive(Clap)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "MD5 Hashing algorithm.")]
pub struct MD5_Opts {
    /// A string or file.
    pub payload: String
}