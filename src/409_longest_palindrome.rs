use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut ret = 0;
        let mut max = 0;
        for &v in map.values() {
            if v % 2 == 0 {
                ret += v;
            } else {
                ret += v - 1;
                max = 1;
            }
        }
        ret + max
    }
}
