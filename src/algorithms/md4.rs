use md4::{Md4, Digest};

pub fn hash(input: &str) -> String {
  let mut hasher = Md4::new();
  hasher.update(input);
  
  format!("{:X}", hasher.finalize())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generates_correct_hash() {
    let result = hash("test");
    assert_eq!(result, "DB346D691D7ACC4DC2625DB19F9E3F52")
  }
}