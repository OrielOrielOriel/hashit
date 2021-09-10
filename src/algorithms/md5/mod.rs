use clap::Clap;
use md5;

use crate::input::check_if_file;

pub fn hash_controller(data: String) -> md5::Digest {
    if check_if_file(&data) {
        hash_string(data)
    } else {
        hash_string(data)
    }
}

fn hash_string(payload: String) -> md5::Digest {
    let ret: md5::Digest = md5::compute(&payload);
    ret
}

// pub fn hash_file(payload: String) -> md5::Digest {

// }

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "md5 Hashing algorithm.")]
pub struct Md5Opts {
    
    #[clap(setting = clap::ArgSettings::TakesValue)]
    pub payload: String,
}