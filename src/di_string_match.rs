pub struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut small = 0;
        let mut large = bytes.len() as i32;
        let mut ret = vec![];
        for &b in bytes {
            if b == 'I' as u8 {
                ret.push(small);
                small += 1;
            } else {
                ret.push(large);
                large -= 1;
            }
        }
        ret.push(small);
        ret
    }
}
