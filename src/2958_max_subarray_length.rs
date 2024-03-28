use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut dict = HashMap::new();
        let mut left = 0;
        let mut max = 0;
        for right in 0..nums.len() {
            let n = nums[right];
            *dict.entry(n).or_insert(0) += 1;
            while dict[&n] > k {
                let m = nums[left];
                *dict.get_mut(&m).unwrap() -= 1;
                left += 1;
            }
            max = max.max(right - left + 1);
        }
        max as i32
    }
}
