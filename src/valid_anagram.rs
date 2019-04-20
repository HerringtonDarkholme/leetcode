// use crate::util::test::test_cases;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut cs = HashMap::new();
        for i in s.chars() {
            *cs.entry(i).or_insert(0) += 1
        }
        for c in t.chars() {
            if let Some(r) = cs.get_mut(&c) {
                if *r <= 0 {
                    return false;
                }
                *r -= 1
            } else {
                return false;
            }
        }
        cs.iter().all(|(_, c)| *c == 0)
    }
}

#[test]
fn test() {
    let f = |s: &str, t: &str| Solution::is_anagram(s.to_owned(), t.to_owned());
    assert_eq!(f("anagram", "nagaram"), true);
    assert_eq!(f("rat", "car"), false);
    assert_eq!(f("aaa", "aa"), false);
    assert_eq!(f("bb", "bbb"), false);
    assert_eq!(f("ccc", "ccc"), true);
}
