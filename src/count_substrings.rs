// leetcode 647
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut ret = len as i32;
        if len >= 2 && bytes[len - 1] == bytes[len - 2] {
            ret += 1;
        }
        for j in 1..len-1 {
            for k in 0..=1 {
                let mut i = j;
                let mut l = k;
                loop {
                    if bytes[i - 1] != bytes[i + l] {
                        break;
                    }
                    ret += 1;
                    if i - 1 > 0 && i + l + 1 < len {
                        i -= 1;
                        l += 2;
                    } else {
                        break;
                    }
                }
            }
        }
        ret
    }
}
