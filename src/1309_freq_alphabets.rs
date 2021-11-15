struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let s: Vec<_> = s.bytes().collect();
        if s.is_empty() {
            return String::new();
        }
        let mut i = 0;
        let mut ret = vec![];
        while i < s.len() {
            let c = s[i];
            if c == b'1' || c == b'2' {
                if i + 2 < s.len() && s[i + 2] == b'#' {
                    let n = (c - b'0') * 10 + (s[i+1] - b'0') - 1;
                    ret.push((b'a' + n) as char);
                    i += 3;
                    continue;
                }
            }
            ret.push((b'a' + c - b'1') as char);
            i += 1;
        }
        ret.into_iter().collect()
    }
}
