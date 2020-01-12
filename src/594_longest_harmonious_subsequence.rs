use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut max_len = 0;
        for i in map.keys() {
            if map.contains_key(&(*i + 1)) {
                max_len = max_len.max(map[i] + map[&(*i+1)]);
            }
        }
        max_len
    }
}
