// leetcode 960
// we are finding a subsequence of matrix colmuns
// rather compare one char, compare all chars on rows
impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let a: Vec<_> = a.into_iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
        let max = a[0].len();
        let mut dp = vec![1; max];
        for i in (0..max-1).rev() {
            for j in i+1..max {
                if a.iter().all(|row| row[i] <= row[j]) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        (max - dp.into_iter().fold(0, usize::max)) as i32
    }
}
