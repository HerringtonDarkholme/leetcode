impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let l = words.len();
        let mut count = vec![0; 26];
        for w in words {
            for c in w.chars() {
                let i = c as usize - 'a' as usize;
                count[i] += 1;
            }
        }
        count.iter().all(|&c| c % l == 0)
    }
}
