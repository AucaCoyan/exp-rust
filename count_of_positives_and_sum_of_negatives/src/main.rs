/* kata https://www.codewars.com/kata/576bb71bbbcf0951d5000044/train/rust

Given an array of integers.
Return an array, where the first element is the count of positives numbers and the second element is sum of negative numbers. 0 is neither positive nor negative.
If the input is an empty array or is null, return an empty array.

Example
For input [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], you should return [10, -65].

*/

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];
    for number in input {
        if number > 0 {
            result[0] = result[0] + 1
        } else if number < 0 {
            result[1] = result[1] + number
        }
    }
    return result;
}

fn main() {
    let my_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -15, -12, -13, -14, -15];
    let my_list2: Vec<i32> = vec![];
    let result = count_positives_sum_negatives(my_list);
    println!("{:?}", result);
    println!("{:?}", count_positives_sum_negatives(my_list2));
}
