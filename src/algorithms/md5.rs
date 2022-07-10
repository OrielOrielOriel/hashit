use md5;

pub fn hash(input: &str) -> String {
    format!("{:X}", md5::compute(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_hash() {
        let result = hash("test");
        assert_eq!(result, "098F6BCD4621D373CADE4E832627B4F6")
    }
}
