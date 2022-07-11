use sha2::{Digest, Sha384};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha384::new();
    hasher.update(input);
    
    format!("{:X}", hasher.finalize())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "768412320F7B0AA5812FCE428DC4706B3CAE50E02A64CAA16A782249BFE8EFC4B7EF1CCB126255D196047DFEDF17A0A9")
    }
}
