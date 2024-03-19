use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut ts = vec![0; 26];
        for task in tasks {
            let i = task as usize - b'A' as usize;
            ts[i] += 1;
        }
        let mut heap: BinaryHeap<_> = ts.into_iter().filter(|n| *n > 0).collect();
        let mut idles = HashMap::new();
        let mut i = 0;
        while !heap.is_empty() || !idles.is_empty() {
            i += 1;
            if let Some(d) = idles.remove(&i) {
                heap.push(d);
            }
            let Some(d) = heap.pop() else {
                continue;
            };
            if d > 1 {
                idles.insert(i + n + 1, d - 1);
            }
        }
        i
    }
}
