use std::collections::HashSet;
impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        let set: HashSet<_> = words.clone().into_iter().collect();
        words.sort_by(|s1, s2| {
            if s1.len() == s2.len() {
                s1.cmp(s2)
            } else {
                s2.len().cmp(&s1.len())
            }
        });
        for w in words {
            let mut found = true;
            for i in 1..=w.len() {
                if !set.contains(&w[0..i]) {
                    found = false;
                    break;
                }
            }
            if found {
                return w
            }
        }
        String::new()
    }
}
