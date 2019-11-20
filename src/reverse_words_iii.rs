// leetcode 557
pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|s| s.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
