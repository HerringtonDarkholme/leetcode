impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let word1: String = word1.into_iter().collect();
        let word2: String = word2.into_iter().collect();
        word1 == word2
    }
}
