impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut chars: Vec<_> = word.chars().collect();
        let Some(mut j) = chars.iter().position(|&r| ch == r) else {
            return word;
        };
        let mut i = 0;
        while i < j {
            chars.swap(i, j);
            i += 1;
            j -= 1;
        }
        chars.into_iter().collect()
    }
}
