pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if nums[mid] > nums[high] {
                low = mid + 1;
            } else if nums[mid] < nums[high] {
                high = mid;
            } else {
                high = high - 1;
            }
        }
        nums[low]
    }
}
