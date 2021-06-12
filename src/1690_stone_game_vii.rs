impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let l = stones.len();
        let mut dp = vec![vec![0; l]; l];
        let mut sum = stones.iter().scan(0, |state, &s| {
            *state += s;
            Some(*state)
        }).collect::<Vec<_>>();
        sum.insert(0, 0);
        for i in (0..l).rev() {
            for j in i+1..l {
                let left = sum[j+1] - sum[i+1] - dp[i+1][j];
                let right = sum[j] - sum[i] - dp[i][j-1];
                dp[i][j] = left.max(right);
            }
        }
        dp[0][l-1]
    }
}
/*
sum[i][j] := prefix sum from i to j
dp[i][j] = the max diff from i to j 
dp[i][i] = 0
dp[i][j] = max(sum[i][j-1] - dp[i][j-1], sum[i+1][j] - dp[i+1][j])
*/
