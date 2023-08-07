impl Solution {
    pub fn min_path_sum(mut dp: Vec<Vec<i32>>) -> i32 {
        // init col
        for i in 1..dp.len() {
            dp[i][0] += dp[i - 1][0];
        }
        // init row
        for i in 1..dp[0].len() {
            dp[0][i] += dp[0][i - 1];
        }
        for i in 1..dp.len() {
            for j in 1..dp[0].len() {
                dp[i][j] += dp[i - 1][j].min(dp[i][j - 1]);
            }
        }
        dp[dp.len() - 1][dp[0].len() - 1]
    }
}
