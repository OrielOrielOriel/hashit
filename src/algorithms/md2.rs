use md2::{Md2, Digest};

pub fn hash(input: &str) -> String {
  let mut hasher = Md2::new();
  hasher.update(input);
  
  format!("{:X}", hasher.finalize())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generates_correct_hash() {
    let result = hash("test");
    assert_eq!(result, "DD34716876364A02D0195E2FB9AE2D1B")
  }
}