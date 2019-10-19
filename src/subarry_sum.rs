pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = HashMap::new();
        prefix.insert(0, 1);
        let mut sum = 0;
        let mut ret = 0;
        for n in nums {
            sum += n;
            if let Some(&other) = prefix.get(&(sum - k)) {
                ret += other;
            }
            *prefix.entry(sum).or_insert(0) += 1;
        }
        ret

    }
}
