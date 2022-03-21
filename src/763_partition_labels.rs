pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut occs = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            let mut interval = occs.entry(c).or_insert((i, i));
            if interval.0 > i {
                interval.0 = i;
            }
            if interval.1 < i {
                interval.1 = i;
            }
        }
        let mut intervals: Vec<_> = occs.values().collect();
        intervals.sort();
        let mut ret = vec![];
        let mut start = 0;
        let mut end = 0;
        for &(s, e) in intervals {
            if end < s {
                ret.push(end - start + 1);
                start = s;
                end = e;
            } else {
                end = e.max(end);
            }
        }
        ret.push(end - start + 1);
        ret
    }
}

/*
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s: Vec<_> = s.bytes().collect();
        let mut occ = vec![0; 26];
        for i in 0..s.len() {
            let c = (s[i] - b'a') as usize;
            occ[c] = i;
        }
        let mut ret = vec![];
        let mut start = 0;
        let mut max = 0;
        for i in 0..s.len() {
            let c = (s[i] - b'a') as usize;
            max = max.max(occ[c]);
            if max == i {
                ret.push((i - start + 1) as i32);
                start = max + 1;
                max = max + 1;
            }
        }
        ret
    }
}
*/
