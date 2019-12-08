pub struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();
        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            if a - b > 0 {
                heap.push(a - b);
            }
        }
        heap.pop().unwrap_or(0)
    }
}
