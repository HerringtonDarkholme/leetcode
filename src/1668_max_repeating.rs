impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let max = sequence.len() / word.len();
        let mut cnt = 0;
        for i in 1..=max {
            if sequence.contains(&(word.repeat(i))) {
                cnt = i;
            }
        }
        cnt as i32
    }
}
