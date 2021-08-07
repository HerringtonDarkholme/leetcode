use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let t = target - n;
            if let Some(&j) = m.get(&t) {
                return vec![j as i32, i as i32]
            }
            m.insert(n, i);
        }
        unreachable!()
    }
}
