use maximum_profit_in_job_searching;
fn main() {
    env_logger::init();
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];

    println!(
        "{}",
        maximum_profit_in_job_searching::Solution::job_scheduling(start_time, end_time, profit)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];

        assert_eq!(
            maximum_profit_in_job_searching::Solution::job_scheduling(start_time, end_time, profit),
            120
        );
    }
}
