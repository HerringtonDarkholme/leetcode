impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut dp = vec![(0, 0)]; // cost, pos
        let len = ring.len();
        let ring = board(ring);
        for k in key.chars() {
            let i = (k as u8 - b'a') as usize;
            dp = rotate(dp, &ring, i, len);
        }
        let ret = dp.into_iter().map(|b| b.0).min().unwrap() + key.len();
        ret as i32
    }
}
// letter -> positions
fn board(ring: String) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![]; 26];
    for (p, c) in ring.bytes().enumerate() {
        let i = (c - b'a') as usize;
        ret[i].push(p);
    }
    ret
}
fn distance(i: usize, j: usize, len: usize) -> usize {
    let (i, j) = (i.min(j), j.max(i));
    (j - i).min(i + len - j)
}
fn rotate(dp: Vec<(usize, usize)>, ring: &Vec<Vec<usize>>, key: usize, len: usize) -> Vec<(usize, usize)> {
    ring[key].iter().map(|&target_pos| {
        let mut min = usize::MAX;
        for &(cost, src_pos) in &dp {
            let d = distance(src_pos, target_pos, len);
            min = min.min(cost + d);
            if min == 0 { break; }
        }
        (min, target_pos)
    }).collect()
}
// |
// godding

/*

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
*/
