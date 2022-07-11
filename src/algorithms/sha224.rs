use sha2::{Digest, Sha224};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha224::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "90A3ED9E32B2AAF4C61C410EB925426119E1A9DC53D4286ADE99A809")
    }
}
