pub mod md5;
pub mod md4;
pub mod md2;
pub mod sha1;
pub mod sha224;
pub mod sha256;
pub mod sha384;
pub mod sha512;
pub mod sha512_224;
pub mod sha512_256;
pub mod ntlm;

use clap::ArgEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algo {
    Md5,
    Md4,
    Md2,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Ntlm,
}
