pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        while n != 0 {
            // below is important because decrement in block is executed before array indexing
            let i = n + m - 1;
            nums1[i] = if m == 0 || nums1[m - 1] < nums2[n - 1] {
                n -= 1;
                nums2[n]
            } else {
                m -= 1;
                nums1[m]
            }
        }
    }
}
