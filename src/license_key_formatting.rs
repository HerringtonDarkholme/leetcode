pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let parts: String = s.split("-").collect();
        let s: Vec<_> = parts.chars().rev().collect();
        s.chunks(k as usize)
            .map(|v| v.into_iter().rev().collect::<String>())
            .rev()
            .collect::<Vec<_>>()
            .join("-")
            .to_uppercase()

    }
}
