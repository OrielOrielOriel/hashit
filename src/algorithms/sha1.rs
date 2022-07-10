use sha1::{Digest, Sha1};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "A94A8FE5CCB19BA61C4C0873D391E987982FBBD3")
    }
}
