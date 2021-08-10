pub struct Solution;

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
impl Solution {
    pub fn nth_super_ugly_number(mut n: i32, primes: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        heap.push(Reverse(1));
        set.insert(1);
        while n > 0 {
            r = heap.pop().unwrap().0;
            heap.extend(primes.iter().filter_map(|&c| {
                let m = c * r;
                if set.contains(&m) {
                    None
                } else {
                    set.insert(m);
                    Some(Reverse(m))
                }
            }));
            n -= 1;
        }
        r
    }
}
