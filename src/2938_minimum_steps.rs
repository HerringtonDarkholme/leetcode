impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut s: Vec<_> = s.bytes().collect();
        let (mut i, mut j) = (0, 0);
        let mut ret = 0;
        while i < s.len() {
            if s[i] == b'0' { 
                i += 1;
                j = j.max(i);
                continue;
            }
            while j < s.len() && s[j] == b'1' {
                j += 1;
            }
            if j == s.len() { break; }
            s.swap(i, j);
            ret += (j - i) as i64;
        }
        ret
    }
}
