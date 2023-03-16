fn add_binary(a: u64, b: u64) -> String {
    /* let result = a + b;
    println!("{}", result);
    let value = format!("{:b}", result);
    println!("{}", value); */
    format!("{:b}", a + b)
}
fn main() {
    add_binary(1, 2);
}

// https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::add_binary;

    fn dotest(a: u64, b: u64, expected: &str) {
        let actual = add_binary(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1, 1, "10");
        dotest(0, 1, "1");
        dotest(1, 0, "1");
        dotest(2, 2, "100");
        dotest(51, 12, "111111");
    }
}
