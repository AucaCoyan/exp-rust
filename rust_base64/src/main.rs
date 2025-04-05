mod lib;

fn main() {}

#[cfg(test)]
mod tests {
    use super::lib::Base64;

    #[test]
    fn test_base64_encode_decode() {
        let original = "TESTING";
        let mut base64 = Base64::new();
        let encoded = base64
            .encode(original)
            .expect("could not encode the string");
        let decoded = base64.decode(&encoded).unwrap();

        assert_eq!(original, decoded, "Original and decoded do not match");
    }
}
