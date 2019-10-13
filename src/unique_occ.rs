pub struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in arr {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut set = HashSet::new();
        for v in map.values() {
            if set.contains(v) {
                return false
            }
            set.insert(*v);
        }
        true
    }
}
