impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut chars = vec![0; 26];
        for c in word.chars() {
            let c = c as u8;
            if c >= b'a' && c <= b'z' {
                let index = (c - b'a') as usize;
                chars[index] |= 1;
            } else if c >= b'A' && c <= b'Z' {
                let index = (c - b'A') as usize;
                chars[index] |= 2;
            }
        }
        chars.into_iter().filter(|c| *c == 3).count() as i32
    }
}
