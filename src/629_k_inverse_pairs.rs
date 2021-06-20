const M: i64 = 1_000_000_007;
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![0i64; k + 1];
        dp[0] = 1;
        for i in 1..n {
            dp = (0..=k).map(|j| {
                let b = j.max(i) - i;
                dp[b..=j].iter().sum::<i64>() % M
            }).collect();
        }
        (dp[k] % M) as i32
    }
}

/*
dp[n][k] the number of length-n arrays that have k inv pair

dp[n][k] = dp[n-1][k] // put min element on arr[0]
         + dp[n-1][k-1] // put 2nd min on arr[0]
         ....
         + dp[n-1][k-n] // put at most n-th min element
*/
