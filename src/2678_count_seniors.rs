impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .map(|s| s.as_bytes())
            .map(|s| (s[11] - b'0') * 10 + (s[12] - b'0') )
            .filter(|age| *age > 60)
            .count() as i32
    }
}
