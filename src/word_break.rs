pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<_> = word_dict.into_iter().collect();
        let max = dict.iter().map(|s| s.len()).fold(0, usize::max);
        let mut dp = vec![false; s.len()+1];
        dp[s.len()] = true;
        for i in 0..s.len() {
            let index = s.len() - i - 1;
            for j in 1..=(i+1).min(max) {
                let word = &s[index..index+j];
                if dict.contains(word) && dp[index+j] {
                    dp[index] = true;
                    break;
                }
            }
        }
        dp[0]
    }
}
