pub struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        if s.is_empty() {
            return 0
        }
        let len = s.len();
        let mut dp = vec![vec![0; len]; len];
        let s: Vec<_> = s.chars().collect();
        find_palindrom(&s, &mut dp)
    }
}

fn find_palindrom(s: &[char], dp: &mut Vec<Vec<i32>>) -> i32 {
    for i in (0..s.len()).rev() {
        dp[i][i] = 1;
        for j in i+1..s.len() {
            if s[i] == s[j] {
                dp[i][j] = dp[i+1][j-1] + 2;
            } else {
                dp[i][j] = dp[i+1][j].max(dp[i][j-1]);
            }
        }
    }
    dp[0][s.len() - 1]
}
