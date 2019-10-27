pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let t: Vec<_> = t.chars().collect();
        let mut i = 0;
        for c in s.chars() {
            while i < t.len() && c != t[i] {
                i += 1;
            }
            if i >= t.len() {
                return false
            }
            i += 1; // advance
        }
        true
    }
}
