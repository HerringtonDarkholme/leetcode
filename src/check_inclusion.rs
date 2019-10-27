pub struct Solution;

const SIZE: usize = 26;
const B: u8 = 'a' as u8;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.is_empty() {
            return !s2.is_empty()
        }
        let mut target = vec![0; SIZE];
        let mut count = 0;
        for &c in s1.as_bytes() {
            target[(c - B) as usize] += 1;
            count += 1;
        }
        let s2 = s2.as_bytes();
        let mut left = 0;
        let mut i = 0;
        while i < s2.len() {
            let c = (s2[i] - B) as usize;
            while target[c] == 0 && left < i {
                target[(s2[left] - B)as usize] += 1;
                left += 1;
                count += 1;
            }
            i += 1;
            if target[c] == 0 {
                left += 1;
                continue;
            }
            target[c] -= 1;
            count -= 1;
            if count == 0 {
                return true
            }
        }
        false
    }
}
