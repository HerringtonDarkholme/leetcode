use std::collections::HashSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: HashSet<_> = nums1.into_iter().collect();
        let nums2: HashSet<_> = nums2.into_iter().collect();
        vec![nums1.difference(&nums2).copied().collect(), nums2.difference(&nums1).copied().collect()]
    }
}

