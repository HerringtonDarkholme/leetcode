pub struct Solution;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct F {
    c: usize,
    w: String,
}

impl Ord for F {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.c.cmp(&other.c) {
            Ordering::Equal => {
                other.w.cmp(&self.w)
            },
            c => c,
        }
    }
}
impl PartialOrd for F {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, mut k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (word, c) in map {
            heap.push(F{c, w:word});
        }
        let mut ret = vec![];
        while k > 0 {
            ret.push(heap.pop().unwrap().w);
            k -= 1;
        }
        ret
    }
}
