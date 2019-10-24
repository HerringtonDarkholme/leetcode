pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums1 {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut ret = vec![];
        for n in nums2 {
            if let Some(c) = map.get_mut(&n) {
                if *c > 0 {
                    ret.push(n);
                    *c -= 1;
                }
            }
        }
        ret
    }
}
