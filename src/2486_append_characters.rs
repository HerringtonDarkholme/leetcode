impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let t = t.as_bytes();
        let mut i = 0;
        for c in s.bytes() {
            if c != t[i] {
                continue;
            }
            i += 1;
            if i == t.len() {
                break;
            }
        }
        (t.len() - i) as i32
    }
}
