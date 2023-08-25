/*
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        // for first n char in s3, can we interleave with m char from s2
        let mut dp = vec![false; s2.len() + 1]; 
        dp[0] = true; // first 0 char in s3 matches 0 char from s2
        for i in 0..s3.len() {
            let mut next = vec![false; s2.len() + 1];
            next[0] = if i < s1.len() { dp[0] && s1[i] == s3[i] } else { false };
            // take j char from s2
            let least = if i + 1 > s1.len() { i + 1 - s1.len() } else { 1 };
            let most = (i+1).min(s2.len());
            for j in least..=most {
                next[j] = 
                    dp[j] && s1[i - j] == s3[i] ||  // add s1 to last
                    dp[j - 1] && s2[j - 1] == s3[i]; // add s2 to last
            }
            dp = next;
        }
        dp[dp.len() - 1]
    }
}
*/

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
