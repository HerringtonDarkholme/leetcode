pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        let mut ans = i32::max_value();
        for i in 0..nums.len() {
            sum += nums[i];
            while sum >= s {
                ans = ans.min((i + 1 - left) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if ans == i32::max_value() { 0 } else { ans }
    }
}
