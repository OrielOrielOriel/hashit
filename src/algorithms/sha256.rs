use sha2::{Digest, Sha256};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "9F86D081884C7D659A2FEAA0C55AD015A3BF4F1B2B0B822CD15D6C15B0F00A08")
    }
}
