use clap::Clap;
use md5;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "md5 Hashing algorithm.")]
pub struct Md5Opts {
    
    #[clap(setting = clap::ArgSettings::TakesValue)]
    pub payload: String,
}