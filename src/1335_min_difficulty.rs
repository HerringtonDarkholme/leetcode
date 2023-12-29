impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let mut d = d as usize;
        let len = job_difficulty.len();
        if len < d { return -1; }
        let mut dp = vec![vec![]; d];
        dp[0].push((job_difficulty[0], job_difficulty[0]));
        for i in 1..len {
            let diff = job_difficulty[i];
            let mut next = vec![vec![]; d];
            let n = dp[0][0].0.max(diff);
            next[0].push((n, n));
            for j in 1..d {
                if dp[j - 1].is_empty() {
                    continue;
                }
                let last = dp[j - 1].last().unwrap();
                let add_new = last.0 + diff;
                let mut add_exist = 300 * 1000 + 1;
                for &(total, max) in &dp[j] {
                    if max >= diff {
                        next[j].push((total, max));
                    } else {
                        add_exist = add_exist.min(total + diff - max);
                    }
                }
                let new_total = add_exist.min(add_new);
                let nn = next[j].len();
                if nn == 0 || next[j][nn - 1].0 > new_total {
                    next[j].push((new_total, diff));
                }
            }
            dp = next;
        }
        dp[d - 1].last().unwrap().0
    }
}
// 1 3 5 4 3 N 1
// 1 2 3 3 N N 
// 1 2 4 1 0 2
