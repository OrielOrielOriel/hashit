use ntlm_hash::ntlm_hash;

pub fn hash(input: &str) -> String {
  ntlm_hash(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generates_correct_hash() {
    let result = hash("test");
    assert_eq!(result, "0cb6948805f797bf2a82807973b89537")
  }
}