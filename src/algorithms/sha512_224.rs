use sha2::{Digest, Sha512_224};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha512_224::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "06001BF08DFB17D2B54925116823BE230E98B5C6C278303BC4909A8C")
    }
}
