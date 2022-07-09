use md5;

fn hash(input: String) -> u8 {
  md5::compute(&input)
}
