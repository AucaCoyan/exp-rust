fn is_square(n: i64) -> bool {
    if n < 0 {
        println!("returned false");
        false
    } else if n == 0 {
        true
    } else {
        for number in 1..n {
            println!("number is {number}");
            let number_squared = number * number;
            println!("squared is {number_squared}");
            if number * number == n {
                println!("returned true");
                return true;
            } else {
                continue;
            }
        }
        // this number either has no squares or the for loop is empty
        println!("returned false");
        false
    }
}

fn main() {
    is_square(26);
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_square;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            is_square(-1),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(0),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(3),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(4),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(25),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(26),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
