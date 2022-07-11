use sha2::{Digest, Sha512_256};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha512_256::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "3D37FE58435E0D87323DEE4A2C1B339EF954DE63716EE79F5747F94D974F913F")
    }
}
