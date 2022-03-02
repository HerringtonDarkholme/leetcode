impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut cs = s.chars().peekable();
        for c in t.chars() {
            if let Some(&s) = cs.peek() {
                if s == c {
                    cs.next();
                }
            } else {
                break;
            }
        }
        cs.peek().is_none()
    }
}
