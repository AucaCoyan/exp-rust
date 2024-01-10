use log::info;
pub struct Solution {}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        // idx = 1
        // let mut idx = 0;
        let mut path: Vec<i32> = vec![];
        // take 1 job
        let range = 1..start_time.len();
        for job_num in range {
            info!("checking the job {job_num}");
            // check the next job
            //     does it overlap?
            if start_time[job_num + 1] < end_time[job_num] {
                //         yes: skip it
                info!("the job {} starts before the job {}", job_num + 1, job_num);
                continue;
            } else {
            };
            //         no: add it to todo_list
            //     continue
        }
        // finished? idx += 1 and continue
        1
    }
}
