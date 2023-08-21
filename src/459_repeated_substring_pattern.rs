impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        (1..=(s.len() / 2))
            .filter(|len| s.len() % len == 0)
            .any(|len| can_divide(s, len))
    }
}
#[inline(always)]
fn can_divide(s: &[u8], len: usize) -> bool {
    for i in 1..(s.len() / len) {
        if s[0..len] != s[i * len..i*len+len] {
            return false;
        }
    }
    true
}
