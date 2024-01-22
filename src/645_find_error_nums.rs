impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut sum = nums[0];
        let mut dup = 0;
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                dup = nums[i];
            }
            sum += nums[i];
        }
        let n = nums.len() as i32;
        vec![dup, n * (n + 1) / 2 - sum + dup]
    }
}
