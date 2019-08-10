pub struct Solution;

impl Solution {
    // optionally we can maintain two pointers to swap bytes
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}
