impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        while n > 0 {
            if m == 0 || nums1[m - 1] <= nums2[n - 1] {
                 nums1[m + n - 1] = nums2[n - 1];
                 n -= 1;
            } else {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            }
        }
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        for i in (0..m).rev() {
            nums1[n + i] = nums1[i];
        }
        let mut i = n;
        let mut j = 0;
        let mut c = 0;
        while j < n && i < m + n {
            if nums1[i] < nums2[j] {
                nums1[c] = nums1[i];
                i += 1;
            } else {
                nums1[c] = nums2[j];
                j += 1;
            }
            c += 1;
        }
        while j < n {
            nums1[c] = nums2[j];
            c += 1;
            j += 1;
        }
    }
}
