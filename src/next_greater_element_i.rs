pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums1.len()];
        let map = build_hashmap(nums1);
        let mut stack = vec![];
        for n in nums2 {
            while !stack.is_empty() && stack[stack.len() - 1] < n {
                let last = stack.pop().unwrap();
                if let Some(&i) = map.get(&last) {
                    ret[i] = n;
                }
            }
            stack.push(n);
        }
        ret
    }
}

fn build_hashmap(nums: Vec<i32>) -> HashMap<i32, usize> {
    let mut ret = HashMap::new();
    for (k, &n) in nums.iter().enumerate() {
        ret.insert(n, k);
    }
    ret
}
