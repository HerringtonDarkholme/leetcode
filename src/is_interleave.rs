// leetcode 97
pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<_> = s1.chars().collect();
        let s2: Vec<_> = s2.chars().collect();
        let s3: Vec<_> = s3.chars().collect();
        if s1.len() + s2.len() != s3.len() {
            return false
        }
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for i in 0..s1.len() {
            if s1[i] == s3[i] {
                dp[i + 1][0] =  true;
            } else {
                break;
            }
        }
        for i in 0..s2.len() {
            if s2[i] == s3[i] {
                dp[0][i + 1] =  true;
            } else {
                break;
            }
        }
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                dp[i][j] = (
                    s1[i - 1] == s3[i + j - 1] && dp[i - 1][j]
                ) || (
                    s2[j - 1] == s3[i + j - 1] && dp[i][j - 1]
                )
            }
        }
        dp[s1.len()][s2.len()]
    }
}
