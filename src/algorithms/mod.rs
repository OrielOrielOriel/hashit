pub mod md5;
pub mod sha1;
use clap::ArgEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algo {
    Md5,
    Sha1,
}
