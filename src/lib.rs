//! hashit lets you hash data using a wide range of hashing algorithms.
mod algorithms;

pub struct Hash {
    pub payload: (String, bool),
    pub algorithm: algorithms::Algorithm,
}