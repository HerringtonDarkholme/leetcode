pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut j = 0;
        let mut k = 0; // record how many zeroes has met
        for i in nums.iter() {
            k += i - 1;
            if k < 0 {
                if nums[j] == 0 {
                    k += 1;
                }
                j += 1;
            }
        }
        (nums.len() - j) as i32
    }
}
