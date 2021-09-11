use clap::Clap;
pub mod md5;

#[derive(Clap, Debug)]
pub enum Algorithms_List {
    md5(crate::algorithms::md5::Md5Opts),
}


