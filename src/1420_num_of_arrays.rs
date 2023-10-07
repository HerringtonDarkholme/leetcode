const M: i64 = 1_000_000_007;
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        if k == 0 {
            return 0 // impossible to avoid max bump
        }
        let (n, m, k) = (n as usize, m as usize, k as usize);
        // num of ways at index i with bump-times b of max value v
        let mut dp = vec![vec![vec![0i64; m]; k]; n];
        for v in 0..m {
            dp[0][0][v] = 1; // initialize first bump
        }
        for i in 1..n {
            for v in 0..m { // initialize only one bump 
                dp[i][0][v] = dp[i-1][0][v] * (v as i64 + 1) % M;
            }
            for b in 1..k {
                for v in 0..m {
                    let no_bump = dp[i-1][b][v] * (v as i64 + 1) % M;
                    let bump: i64 = dp[i-1][b-1].iter().take(v).sum(); // no overflow
                    dp[i][b][v] = ((bump % M) + no_bump) % M;
                }
            }
        }
        (dp[n-1][k-1].iter().sum::<i64>() % M)as i32
    }
}
