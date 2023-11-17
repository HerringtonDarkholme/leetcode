impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let len = nums.len();
        let mut min = 0;
        for i in 0..len/2 {
            min = min.max(nums[i] + nums[len - 1 - i]);
        }
        min
    }
}
