pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut low = 0;
        let mut high = n - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if mid < n -1 && nums[mid] < nums[mid+1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}
