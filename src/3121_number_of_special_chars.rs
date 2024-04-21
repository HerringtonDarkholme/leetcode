impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        // 0-> not seen 1-> seen low 2-> seen upper, 3 -> impos
        let mut chars = vec![0; 26];
        for c in word.chars() {
            let c = c as u8;
            if c >= b'a' && c <= b'z' {
                let index = (c - b'a') as usize;
                if chars[index] <= 1 {
                    chars[index] = 1;
                } else {
                    chars[index] = 3;
                }
            } else if c >= b'A' && c <= b'Z' {
                let index = (c - b'A') as usize;
                if chars[index] == 1 || chars[index] == 2 {
                    chars[index] = 2;
                } else {
                    chars[index] = 3;
                }
            }
        }
        chars.into_iter().filter(|&c| c == 2).count() as i32
    }
}
