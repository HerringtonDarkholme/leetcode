impl Solution {
    fn longest_palindrome_subseq(s: &str) -> usize {
        let n = s.len();
        let mut dp = vec![0; n];
        let mut dp_prev = vec![0; n];

        for start in (0..n).rev() {
            dp[start] = 1;
            for end in start + 1..n {
                if s.as_bytes()[start] == s.as_bytes()[end] {
                    dp[end] = dp_prev[end - 1] + 2;
                } else {
                    dp[end] = dp_prev[end].max(dp[end - 1]);
                }
            }
            dp_prev = dp.clone();
        }

        dp[n - 1]
    }

    pub fn min_insertions(s: String) -> i32 {
        (s.len() - Self::longest_palindrome_subseq(&s)) as i32
    }
}
