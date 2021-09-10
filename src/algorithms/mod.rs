use clap::Clap;
pub mod md5;

#[derive(Clap, Debug)]
pub enum Algorithm {
    md5(crate::algorithms::md5::Md5Opts),
}


