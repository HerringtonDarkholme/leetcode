use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dp = vec![vec![]; s.len() + 1];
        dp[0] = vec![vec![]];
        let dict: HashSet<_> = word_dict.into_iter().collect();
        for end in 0..s.len() {
            let mut curr = vec![];
            for start in 0..=end {
                let sub = &s[start..=end];
                if !dict.contains(&*sub) { continue; }
                for v in &dp[start] {
                    let mut v = v.clone();
                    v.push(sub);
                    curr.push(v);
                }
            }
            dp[end + 1] = curr;
        }
        dp[s.len()].iter().map(|v| {
            v.join(" ")
        }).collect()
    }
}
