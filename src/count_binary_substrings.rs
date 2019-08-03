pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;
        let mut preced = 0;
        let mut after = 1;
        let mut last_c = s[0];
        for &c in s[1..].iter() {
            if c == last_c {
                after += 1;
                continue;
            }
            ret += std::cmp::min(preced, after);
            preced = after;
            last_c = c;
            after = 1;
        }
        ret += std::cmp::min(preced, after);
        ret
    }
}
