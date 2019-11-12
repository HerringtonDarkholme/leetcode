// leetcode 647
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut candidates = vec![];
        for i in 1..len {
            candidates.push((i, 0));
            candidates.push((i, 1));
        }
        candidates.pop();
        let mut ret = len as i32;
        while !candidates.is_empty() {
            let (i, l) = candidates.pop().unwrap();
            if bytes[i - 1] != bytes[i + l] {
                continue;
            }
            ret += 1;
            if i - 1 > 0 && i + l + 1 < len {
                candidates.push((i - 1, l + 2));
            }
        }
        ret
    }
}
