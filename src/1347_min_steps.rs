impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut letters = vec![0; 26];
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            letters[i] += 1;
        }
        for b in t.bytes() {
            let i = (b - b'a') as usize;
            if letters[i] > 0 {
                letters[i] -= 1;
            }
        }
        letters.into_iter().sum()
    }
}
