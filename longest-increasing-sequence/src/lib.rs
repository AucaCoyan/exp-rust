pub struct Solution {}

// Constraints:
//
//    1 <= nums.length <= 2500
//    -10^4 <= nums[i] <= 10^4

impl Solution {
    // first try:
    //
    // You may want to use peek, but peek is not (very) useful in a for loop
    // the problem occurs when you use a for loop, it makes a mutable reference
    // and you can't borrow it again.
    //
    // pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    //         let mut len = 1;
    //         let mut peekable = nums.iter().peekable();
    //         for number in peekable {
    //             // 👇 this gives you an error of a moved value
    //             if peekable.peek().is_some() && number > peekable.peek().unwrap() {
    //                 println!("yes!");
    //                 len = len + 1
    //             }
    //         }
    //         len
    //     }

    // the possible solution with peekable is to use a while loop
    pub fn length_of_lis_using_peek_while(nums: Vec<i32>) -> i32 {
        let mut vec_len: Vec<i32> = vec![1];
        let mut vec_idx = 0;
        let mut peekable = nums.iter().peekable();
        while let Some(number) = peekable.next() {
            if number < peekable.next().unwrap() {
                vec_len[vec_idx] += 1;
            } else {
                vec_len.push(1);
                vec_idx += 1;
            }
        }
        let result = *vec_len.iter().max().unwrap();
        result
    }

    // there is another method from itertools called `windows(n)`
    // which gives you a block of `n` numbers
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut vec_len: Vec<i32> = vec![1];
        let mut vec_idx = 0;
        for number in nums.windows(2) {
            // you don't need to check if the next value is some! 🙌
            if number[0] < number[1] {
                vec_len[vec_idx] += 1;
            } else {
                vec_len.push(1);
                vec_idx = 0;
            }
        }
        let result = *vec_len.iter().max().unwrap();
        result
    }
}
