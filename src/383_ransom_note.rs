use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let ransom = from_str(ransom_note);
        let magazine = from_str(magazine);
        for (k, i) in ransom.into_iter() {
            if *magazine.get(&k).unwrap_or(&0) < i {
                return false
            }
        }
        true
    }
}
fn from_str(s: String) -> HashMap<char, i32> {
    let mut ret = HashMap::new();
    for c in s.chars() {
        *ret.entry(c).or_insert(0) += 1;
    }
    ret
}
