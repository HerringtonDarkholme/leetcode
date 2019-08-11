pub struct Solution;

use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut heap: BinaryHeap<_> = map.into_iter().map(|(n, c)| (c, n)).collect();
        (0..k).filter_map(|_| heap.pop()).map(|(_, n)| n).collect()
    }
}
