use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut sums = HashMap::new();
        sums.insert(0, 0);
        for (i, n) in nums.into_iter().enumerate() {
            sum += n;
            if k != 0 {
                sum %= k;
            }
            if let Some(j) = sums.get(&sum) {
                if i - j > 0 {
                    return true
                }
            } else {
                sums.insert(sum, i+1);
            }
        }
        false
    }
}
