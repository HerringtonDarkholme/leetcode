use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut ret = HashSet::new();
        for n in nums {
            if set.contains(&(n + k)) {
                ret.insert((n, n + k));
            }
            if set.contains(&(n - k)) {
                ret.insert((n - k, n));
            }
            set.insert(n);
        }
        ret.len() as i32
    }
}

/*
use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        if k == 0 {
            return map.values().filter(|&&v| v > 1).count() as i32
        }
        let mut ret = 0;
        for (n ,v) in map.iter() {
            if map.contains_key(&(n + k)) {
                ret += 1;
            }
        }
        ret
    }
}

*/
