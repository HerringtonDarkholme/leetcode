impl Solution {
    pub fn max_score(s: String) -> i32 {
        // score = lz + ro = len - curr  - tz + 2 * lz
        // let acc = 2 * lz - curr 
        let mut lz = 0;
        let mut acc = -1000 as i32;
        let s = s.as_bytes();
        for curr in 1..s.len() {
            if s[curr - 1] == b'0' {
                lz += 1;
            }
            acc = acc.max(lz * 2 - curr as i32);
        }
        let tz = lz + if s[s.len() - 1] == b'0' { 1 } else { 0 };
        s.len() as i32 - tz + acc
    }
}
