pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.chars().all(char::is_uppercase) {
            true
        } else {
            let mut chars = word.chars();
            chars.next();
            chars.all(char::is_lowercase)
        }
    }
}
