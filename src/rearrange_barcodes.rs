pub struct Solution;

// for this question we can specialize the solution to assign odd/even position
// alternatively, we can generalize the problem to k-distance apart elements
use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(barcodes.len());
        let mut map = HashMap::new();
        for b in barcodes {
            *map.entry(b).or_insert(0) += 1;
        }
        let mut heap: BinaryHeap<_> = map
            .into_iter()
            .map(|(b, occ)| (occ, b))
            .collect();
        while !heap.is_empty() {
            let (occ, b) = heap.pop().unwrap();
            ret.push(b);
            let top2 = heap.pop();
            if top2.is_none() {
                break;
            }
            let (occ1, b1) = top2.unwrap();
            ret.push(b1);
            if occ > 1 {
                heap.push((occ - 1, b));
            }
            if occ1 > 1 {
                heap.push((occ1 - 1, b1));
            }
        }
        ret
    }
}
