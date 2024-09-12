impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        words.into_iter().filter(|w| {
            w.chars().all(|c| allowed.contains(c))
        }).count() as i32
    }
}
