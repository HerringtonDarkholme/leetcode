impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut changed = false;
        for i in 1..nums.len() {
            if nums[i - 1] <= nums[i] {
                continue;
            }
            if changed {
                return false;
            }
            if i == 1 ||  nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
            changed = true;
        }
        true
    }
}
