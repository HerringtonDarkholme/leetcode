impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|w| w.len());
        words.reverse();
        let mut encoding = String::new();
        for mut word in words {
            word.push('#');
            if encoding.contains(&word) {
                continue;
            }
            encoding.push_str(&word);
        }
        encoding.len() as i32
    }
}
