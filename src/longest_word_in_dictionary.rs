pub struct Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        d.sort();
        d.sort_by_key(|s| Reverse(s.len()));
        let chars = s.chars().collect::<Vec<_>>();
        for s1 in d {
            let s1 = s1.chars().collect::<Vec<_>>();
            let mut i = 0;
            let mut j = 0;
            while i < chars.len() && j < s1.len() {
                if chars[i] == s1[j] {
                    i += 1;
                    j += 1;
                } else {
                    i += 1;
                }
            }
            if j == s1.len() {
                return s1.iter().collect()
            }
        }
        String::new()
    }
}
