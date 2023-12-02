impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut freqs = vec![0; 26];
        for c in chars.bytes() {
            freqs[(c - b'a') as usize] += 1;
        }
        let mut ret = 0;
        for word in words {
            let mut cnt = 0;
            let mut chars = freqs.clone();
            for c in word.bytes() {
                let i = (c - b'a') as usize;
                if chars[i] == 0 {
                    cnt = -1;
                    break;
                }
                chars[i] -= 1;
                cnt += 1;
            }
            if cnt >= 0 {
                ret += cnt;
            }
        }
        ret
    }
}
