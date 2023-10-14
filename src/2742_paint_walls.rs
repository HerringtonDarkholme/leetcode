impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        // Knapsack problem, given time&paid walls, which costs least?
        let len = cost.len();
        let mut dp = vec![-1; len + 1];
        dp[0] = 0;
        for i in 0..cost.len() { // i-th wall to pay
            let mut next = dp.clone();
            for j in 0..=cost.len() { // j hours spent + paid walls
                if dp[j] == -1 { continue; } // not get there yet
                let nj = len.min(j + time[i] as usize + 1);
                let existing = next[nj];
                if existing < 0 {
                    next[nj] = dp[j] + cost[i];
                } else {
                    next[nj] = existing.min(dp[j] + cost[i]);
                }
            }
            dp = next;
        }
        dp[len]
    }
}
