use sha2::{Digest, Sha512};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "EE26B0DD4AF7E749AA1A8EE3C10AE9923F618980772E473F8819A5D4940E0DB27AC185F8A0E1D5F84F88BC887FD67B143732C304CC5FA9AD8E6F57F50028A8FF")
    }
}
