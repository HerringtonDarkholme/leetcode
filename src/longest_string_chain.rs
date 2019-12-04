// leetcode 1048
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        words.sort_by_key(|w| w.len());
        let words = words.into_iter().map(|c| c.chars().collect());
        for w in words {
            let max = map.iter()
                .filter_map(|(pw, &c)| if is_prev(pw, &w) {
                    Some(c + 1)
                } else {
                    None
                }).fold(1, i32::max);
            map.insert(w, max);
        }
        map.values().cloned().fold(0, i32::max)
    }
}
fn is_prev(prev: &Vec<char>, next: &Vec<char>) -> bool {
    if prev.len() != next.len() - 1 {
        return false
    }
    let mut i = 0;
    let mut j = 0;
    while i < prev.len() {
        if prev[i] == next[j] {
            i += 1;
            j += 1;
        } else if i == j {
            j += 1;
        } else {
            return false
        }
    }
    true
}
