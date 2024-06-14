/// 1235. Maximum profit in job scheduling
/// https://leetcode.com/problems/maximum-profit-in-job-scheduling

#[allow(dead_code)]
#[derive(Debug)]
struct Job {
    start: i32,
    end: i32,
    profit: i32,
}

impl Job {
    fn new(start: i32, end: i32, profit: i32) -> Self {
        Job { start, end, profit }
    }

    fn can_be_combined(&self, job: &Job) -> bool {
        !(job.start > self.start && job.start < self.end)
            || (job.end > self.start && job.end < self.end)
    }
}

pub struct Solution {}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let main_job = Job::new(start_time[0], end_time[0], profit[0]);
        for job_num in 1..start_time.len() {
            let alternative_job = Job::new(start_time[job_num], end_time[job_num], profit[job_num]);
            if main_job.can_be_combined(&alternative_job) {
                println!(
                    "it can be combined! start: {}, {}, end: {}, {}, profit: {}",
                    main_job.start,
                    alternative_job.start,
                    main_job.end,
                    alternative_job.end,
                    (main_job.profit + alternative_job.profit)
                );
            }
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];

        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120);
    }
}
