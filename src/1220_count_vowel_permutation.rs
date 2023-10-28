const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let (a, e, i, o, u) = (0, 1, 2, 3, 4);
        let mut dp = [1, 1, 1, 1, 1];
        for _ in 1..n {
            dp = [
                dp[e], // a + e
                (dp[a] + dp[i]) % M, // e + (a|i)
                (dp[a] + dp[e] + dp[o] + dp[u]) % M, // i + [^i]
                (dp[i] + dp[u]) % M,  // o + (i | u)
                dp[a], // u + a
            ];
        }
        ((dp[a]+dp[e]+dp[i]+dp[o]+dp[u]) % M) as i32
    }
}
