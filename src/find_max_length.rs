// leetcode 525
pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut net = vec![-1; nums.len() * 2 + 1];
        net[len] = 0;
        let mut max = 0;
        let mut sum = 0i32;
        for i in 0..nums.len() {
            let n = nums[i];
            if n == 1 {
                sum += 1;
            } else {
                sum -= 1;
            }
            let idx = (sum + len as i32) as usize;
            if net[idx] != -1 {
                max = max.max(i as i32 + 1 - net[idx]);
            } else {
                net[idx] = i as i32 + 1;
            }
        }
        max
    }
}
