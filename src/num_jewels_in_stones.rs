// 771

pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let j: HashSet<_> = j.chars().collect();
        s.chars().filter_map(|s| j.get(&s).map(|x| 1)).sum()
    }
}
