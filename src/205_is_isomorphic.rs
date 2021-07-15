pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        if s.len() != t.len() {
            return false
        }
        let mut hash1 = std::collections::HashMap::new();
        let mut hash2 = std::collections::HashMap::new();
        for i in 0..s.len() {
            if *hash1.entry(s[i]).or_insert(t[i]) != t[i] ||
               *hash2.entry(t[i]).or_insert(s[i]) != s[i] {
                return false
            }
        }
        return true
    }
}

/*

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s1 = HashMap::new();
        let mut s2 = HashSet::new();
        assert_eq!(s.len(), t.len());
        for (cs, ct) in s.chars().zip(t.chars()) {
            if let Some(&c) = s1.get(&cs) {
                if c != ct {
                    return false
                }
            } else if s2.contains(&ct) {
                return false
            } else {
                s1.insert(cs, ct); 
                s2.insert(ct);
            }
        }
        true
    }
}
*/
