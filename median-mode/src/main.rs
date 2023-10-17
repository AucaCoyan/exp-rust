/// Given a list of integers, use a vector and return
/// the median (when sorted, the value in the middle position)
///  and mode (the value that occurs most often; a hash map will be helpful here)
///  of the list.
///
/// Link of the problem:
/// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary
use env_logger;
use log::trace;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    let my_list = vec![99, 1, 100, 1, 1, 1];
    println!("{:?}", my_list);

    let my_median = median(&my_list);
    println!("the median is: {my_median}");

    let mut value_counts: HashMap<&i32, i32> = HashMap::new();
    let (my_mode, times) = mode(&my_list, value_counts);
    println!("the mode is: {my_mode} with {times} times");
}

fn median(list: &Vec<i32>) -> i32 {
    let mut copy_list = list.clone();
    copy_list.sort();
    trace!("sorted list {:?}", copy_list);
    let middle_element = copy_list.len() / 2;
    copy_list.get(middle_element).unwrap().clone()

    // this is unnecesary, on both cases returns the correct "middle"
    // if there are pair elements, the middle -1
    // if there are impair elements, the middle
    //
    // match list.len() % 2 == 0 {
    //     // pair elements
    //     true => {
    //         let middle_element = list.len() / 2;
    //         list.get(middle_element).unwrap().clone()
    //     }
    //     // impair elements
    //     false => {
    //         let middle_element = list.len() / 2;
    //         list.get(middle_element).unwrap().clone()
    //     }
    // }
}

fn mode(list: &Vec<i32>, value_counts: &mut HashMap<&i32, i32>) -> (i32, i32) {
    for element in list.iter() {
        value_counts
            .entry(element) // find the key
            .and_modify(|counter| *counter += 1) // if it exists, mutate in place and add 1
            .or_insert(1); // if it doesn't, just place 1
    }
    trace!("final value_counts: {:?}", value_counts);

    let mode = &value_counts.values().max().unwrap();
    // let mode = &1;
    let times = 2;
    (mode, times)
}
