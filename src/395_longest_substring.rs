pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s: Vec<_> = s.chars().collect();
        aux(&s, k as usize) as i32
    }
}

fn aux(s: &[char], k: usize) -> usize {
    let map = compute_hash(s);
    for (c, v) in map.iter() {
        if v.len() < k {
            let mut start = 0;
            let mut max = 0;
            for &i in v.iter() {
                max = max.max(aux(&s[start..i], k));
                start = i + 1;
            }
            max = max.max(aux(&s[start..], k));
            return max
        }
    }
    s.len()
}

fn compute_hash(s: &[char]) -> HashMap<char, Vec<usize>> {
    let mut map = HashMap::new();
    for (i, &c) in s.iter().enumerate() {
        map.entry(c).or_insert(vec![]).push(i);
    }
    map
}
