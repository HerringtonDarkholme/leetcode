pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut map = HashMap::new();
        let mut ret = 0;
        let k = k as usize;
        while end <= a.len() {
            // fill map
            while end < a.len() && map.len() < k {
                *map.entry(a[end]).or_insert(0) += 1;
                end += 1;
            }
            if map.len() < k {
                break;
            }
            let initial_e = end;
            end -= 1;
            while end < a.len() && map.contains_key(&a[end]) {
                ret += 1;
                end += 1;
            }
            *map.get_mut(&a[start]).unwrap() -= 1;
            if map[&a[start]] == 0 {
                map.remove(&a[start]);
            }
            start += 1;
            end = initial_e;
        }
        ret
    }
}
