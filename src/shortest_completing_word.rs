pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let chars = license_plate.to_lowercase().chars().filter(|&c| c.is_alphabetic()).fold(HashMap::new(), |mut m, c| {
           *m.entry(c).or_insert(0) += 1;
            m
        });
        let mut ret = "1234567890123456".to_owned();
        'outer: for word in words {
            let w = word.to_lowercase().chars().fold(HashMap::new(), |mut m, c| {
                *m.entry(c).or_insert(0) += 1;
                m
            });
            for (c, &count) in chars.iter() {
                if *w.get(c).unwrap_or(&0) < count {
                    continue 'outer;
                }
            }
            if word.len() < ret.len() {
                ret = word;
            }
        }
        ret
    }
}
