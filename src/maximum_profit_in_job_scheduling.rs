// leetcode 1235
pub struct Solution;

struct Job {
    start: i32,
    end: i32,
    earn: i32,
}

struct Period {
    start: i32,
    max: i32,
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<_> = (0..start_time.len()).map(|i| {
            Job {
                start: start_time[i],
                end: end_time[i],
                earn: profit[i],
            }
        }).collect();
        jobs.sort_by_key(|job| -job.start);
        let mut dp = vec![Period {
            start: i32::max_value(),
            max: 0,
        }];
        let mut max = 0;
        for i in 0..jobs.len() {
            let Job{start, end, earn} = jobs[i];
            let max_profit = earn + search_end(&dp, end);
            if max_profit > max {
                max = max_profit;
                if dp.last().unwrap().start == start {
                    dp.last_mut().unwrap().max = max;
                    continue;
                }
                dp.push(Period {
                    start, max,
                });
            }
        }
        max
    }
}
fn search_end(dp: &Vec<Period>, end: i32) -> i32 {
    let mut low = 0;
    let mut high = dp.len() - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        let period = &dp[mid];
        if period.start > end {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if dp[low].start < end {
        dp[low - 1].max
    } else {
        dp[low].max
    }
}
