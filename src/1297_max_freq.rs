use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s: Vec<_> = s.chars().collect();
        let mut ret = HashMap::new();
        for i in 0..s.len() {
            let j = i + min_size as usize - 1;
            if j >= s.len() {
                break;
            }
            let letters: HashSet<_> = s[i..=j].iter().collect();
            if letters.len() as i32 > max_letters {
                continue;
            }
            *ret.entry(&s[i..=j]).or_insert(0) += 1;
        }
        ret.into_values().max().unwrap_or(0)
    }
}
