use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::new();
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            set.insert(c);
        }
        for c in sentence.chars() {
            set.remove(&c);
        }
        set.is_empty()
    }
}
