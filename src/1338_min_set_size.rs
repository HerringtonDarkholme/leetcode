use std::collections::{HashMap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let l = arr.len();
        for a in arr {
            *count.entry(a).or_insert(0) += 1;
        }
        let mut occ: Vec<_> = count.values().collect();
        occ.sort();
        let mut removed = 0;
        for i in (0..occ.len()).rev() {
            removed += occ[i];
            if removed >= l/2 {
                return (occ.len() - i) as i32;
            }
        }
        unreachable!()
    }
}
