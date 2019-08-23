pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0
        }
        let mut longest = vec![0; len];
        let mut l = 1;
        longest[0] = nums[len - 1];
        for i in 2..=len {
            if longest[l -1] > nums[len-i] {
                longest[l] = nums[len-i];
                l += 1;
            }
            for j in 2..=l {
                if longest[l - j] > nums[len -i] && nums[len - i] > longest[l - j + 1] {
                    longest[l - j + 1] = nums[len - i];
                }
            }
            if longest[0] < nums[len - i] {
                longest[0] = nums[len - i];
            }
        }
        l as i32
    }
}
