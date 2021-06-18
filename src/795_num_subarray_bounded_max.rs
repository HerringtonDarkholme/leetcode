impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut nearest_gte_left = 0;
        let mut nearest_gt_right = 0;
        let mut r = 0;
        for i in 0..nums.len() {
            let n = nums[i];
            if n >= left {
                nearest_gte_left = i + 1;
            }
            if n > right {
                nearest_gt_right = i + 1;
            }
            r += nearest_gte_left - nearest_gt_right;
        }
        r as i32
    }
}
