use log::info;

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
