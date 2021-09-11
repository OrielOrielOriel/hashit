use clap::Clap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use md5::{Md5, Digest};

fn hash_string(payload: &String) -> String {
    let mut hasher = Md5::new();
    hasher.update(payload);
    let ret: String = format!("{:x}", hasher.finalize());
    ret
}

// fn hash_file(payload: &String) -> String {
//     let mut hasher = Md5::new();
//     let file = match File::open(payload) {
//         Ok(file) => file,
//         Err(err) => {
//             println!("my error message: {}", err);
//             std::process::exit(1);
//         }
//     };
//     let mut reader = BufReader::new(file);
//     hasher.update(reader);
//     let ret: String = format!("{:X}", hasher.finalize());
//     ret
// }

pub fn hash_controller(payload: &String) -> () {
    // let path = Path::new(payload);
    // if path.is_file() {
    //     let digest = hash_file(payload);
    //     println!("{:?}", digest);
    // } else {
    //     let digest = hash_string(payload);
    //     println!("{:?}", digest);
    // }

    let hash: String = hash_string(payload);
    println!("{}", hash);
}

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Oriel <Orianafarrugia3@gmail.com>", about = "md5 Hashing algorithm.")]
pub struct Md5Opts {
    
    #[clap(setting = clap::ArgSettings::TakesValue)]
    pub payload: String,
}