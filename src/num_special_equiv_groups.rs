pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        a.into_iter().map(hash).collect::<HashSet<_>>().len() as i32
    }
}
fn hash(s: String) -> String {
    let mut r = vec![0; 52];
    for (i, c) in s.as_bytes().into_iter().enumerate() {
        let idx = (c - 'a' as u8) as usize + 26 * (i % 2);
        r[idx] += 1;
    }
    r.iter().map(|c| c.to_string()).collect()
}
