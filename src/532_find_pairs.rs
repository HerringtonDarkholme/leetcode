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
