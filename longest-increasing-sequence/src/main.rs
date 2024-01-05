use longest_increasing_sequence::Solution;
fn main() {
    let response = Solution::length_of_lis(vec![1, 2, 3]);
    println!("response: {response}");
}

mod tests {
    use longest_increasing_sequence::Solution;

    #[test]
    fn length_is_zero() {
        let response = Solution::length_of_lis(vec![]);
        assert_eq!(response, 1);
    }

    #[test]
    fn example_1() {
        let response = Solution::length_of_lis(vec![10, 9, 5, 2, 3, 7, 101, 18]);
        assert_eq!(response, 4);
    }

    #[test]
    fn example_2() {
        let response = Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]);
        assert_eq!(response, 4);
    }

    #[test]
    fn example_3() {
        let response = Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7, 7]);
        assert_eq!(response, 1);
    }

    #[test]
    fn example_4() {
        let response = Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]);
        assert_eq!(response, 3);
    }
}
