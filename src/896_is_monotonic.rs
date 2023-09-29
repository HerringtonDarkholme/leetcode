impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut sign = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                if sign < 0 {
                    return false;
                }
                sign = 1
            } else if nums[i] < nums[i - 1] {
                if sign > 0 {
                    return false;
                }
                sign = -1;
            }
        }
        true
    }
}
