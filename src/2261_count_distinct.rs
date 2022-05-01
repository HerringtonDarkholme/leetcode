use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let mut set = HashSet::new();
        let mut left = 0;
        let mut count = 0;
        for right in 0..nums.len() {
            count += if nums[right] % p == 0 { 1 } else { 0 };
            if count > k {
                while nums[left] % p != 0 {
                    left += 1;
                }
                count -= 1;
                left += 1;
            }
            for i in left..=right {
                set.insert(&nums[i..=right]);
            }
        }
        set.len() as i32
    }
}
