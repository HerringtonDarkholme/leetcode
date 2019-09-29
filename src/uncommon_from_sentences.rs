pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut hash = HashMap::new();
        for word in a.split_whitespace() {
            *hash.entry(word).or_insert(0) += 1;
        }
        for word in b.split_whitespace() {
            *hash.entry(word).or_insert(0) += 1;
        }
        hash.into_iter()
            .filter_map(|(w, c)| if c == 1 { Some(w.to_string()) } else { None })
            .collect()
    }
}
