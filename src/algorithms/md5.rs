use md5;

pub fn hash(input: String) -> md5::Digest {
  md5::compute(&input)
}
