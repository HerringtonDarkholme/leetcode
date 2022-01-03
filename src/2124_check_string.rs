impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut has_b = false;
        for c in s.chars() {
            if c == 'a' && has_b {
                return false;
            }
            if c == 'b' {
                has_b = true;
            }
        }
        true
    }
}
