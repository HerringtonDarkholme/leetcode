impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false
        }
        let s1 = s.bytes();
        let t1 = t.bytes();
        let mut v = vec![0; 26];
        for (c1, c2) in s1.zip(t1) {
            let diff = if c1 > c2 {
                26 + c2 - c1
            } else {
                c2 - c1
            } as usize;
            if diff > 0 && diff + v[diff] * 26 > k as usize {
                return false
            }
            v[diff] += 1;
        }
        true
    }
}
