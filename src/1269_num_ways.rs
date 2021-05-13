impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        num_ways(steps as usize, arr_len as usize) as i32
    }
}
const M: usize = 1_000_000_007;
fn num_ways(steps: usize, len: usize) -> usize {
    if len == 1 {
        return 1
    }
    let len = steps.min(len);
    let mut dp = vec![0; len];
    dp[0] = 1;
    dp[1] = 1;
    let mut np = vec![1; len];
    for i in 1..steps {
        np[0] = (dp[0] + dp[1] ) % M;
        for j in 1..len-1 {
            np[j] = (dp[j-1] + dp[j] + dp[j+1]) % M;
        }
        np[len-1] = (dp[len-1] + dp[len-2]) % M;
        std::mem::swap(&mut np, &mut dp);
    }
    dp[0] % M
}
