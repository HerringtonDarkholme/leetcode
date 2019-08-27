pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let r_len = ring.len() as i32;
        let k_len = key.len() as i32;
        let mut map = HashMap::new();
        for (i, c) in ring.chars().into_iter().enumerate() {
            map.entry(c).or_insert(vec![]).push(i as i32);
        }
        let mut key = key.chars().rev();
        let last = key.next().unwrap();
        // record the cost to reach char in key, and corresponding pos in ring
        let mut v: Vec<_> = map[&last].iter().map(|&i| {
            (i, 0) // (pos, cost)
        }).collect();
        let compute_min = |v: &Vec<(i32, i32)>, i: i32| {
            v.iter().fold(i32::max_value(), |m, &(pos, cost)| {
                let diff = (pos - i).abs();
                let step = diff.min(r_len - diff);
                m.min(cost + step)
            })
        };
        for c in key {
            v = map[&c].iter().map(|&i| {
                let min = compute_min(&v, i);
                (i, min)
            }).collect();
        }
        compute_min(&v, 0) + k_len
    }
}
