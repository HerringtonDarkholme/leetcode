impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut v = [0; 26];
        for c in t.bytes() {
            let i = (c - b'a') as usize;
            v[i] += 1;
        }
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            v[i] -= 1;
        }
        for i in 0..26 {
            if v[i] > 0 {
                return (i as u8 + b'a') as char
            }
        }
        '-'
    }
}
