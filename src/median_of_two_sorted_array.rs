pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let x = nums1.len();
        let y = nums2.len();
        let mut low = 0;
        let mut high = x;
        while low <= high {
            let partition_x = (low + high) / 2;
            let partition_y = (x + y + 1) / 2 - partition_x;
            let min_left_x = if partition_x > 0 { nums1[partition_x - 1] } else {i32::min_value()};
            let max_right_x = if partition_x >= x { i32::max_value() } else {nums1[partition_x]};
            let min_left_y = if partition_y > 0 { nums2[partition_y - 1] } else { i32::min_value()};
            let max_right_y = if partition_y >= y { i32::max_value() } else { nums2[partition_y]};

            if min_left_x <= max_right_y && max_right_x >= min_left_y {
                return if (x + y) % 2 == 0 {
                    (std::cmp::max(min_left_x, min_left_y) as f64 / 2.0) + (std::cmp::min(max_right_x, max_right_y) as f64 / 2.0)
                } else {
                    std::cmp::max(min_left_x, min_left_y) as f64
                };
            } else if min_left_x > max_right_y {
                high = partition_x - 1;
            } else {
                low = partition_x + 1;
            }
         }
        0.0
    }
}
