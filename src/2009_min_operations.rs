use std::collections::BTreeSet;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut total = nums.len();
        let set: BTreeSet<_> = nums.into_iter().collect();
        let nums: Vec<_> = set.into_iter().collect(); // build a sorted set
        let mut min = usize::MAX;
        for (left, &n) in nums.iter().enumerate() {
            // find pairing max number and compute how many elements can be reused
            match nums.binary_search(&(total as i32 + n - 1)) {
                Ok(right) => {
                    min = min.min(total - right + left - 1);
                }
                Err(right) => {
                    min = min.min(total - right + left);
                    if right == nums.len() {
                        break;
                    }
                }
            }
        }
        min as i32
    }
}
