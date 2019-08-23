pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut captial = HashMap::new();
        let mut exact = HashSet::new();
        let mut vowels = HashMap::new();
        let is_vowel = |c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        };
        let replace_vowel = |s: &str| s.replace(is_vowel, "_");
        for (i, s) in wordlist.iter().enumerate() {
            exact.insert(s.to_string());
            let key = s.to_lowercase();
            captial.entry(key).or_insert(s.clone());
            let key = replace_vowel(&s.to_lowercase());
            vowels.entry(key).or_insert(i);
        }
        let mut ret = vec![];
        for q in queries {
            if exact.contains(&q) {
                ret.push(q);
                continue;
            }
            let q = q.to_lowercase();
            if let Some(&hit) = captial.get(&q) {
                ret.push(wordlist[hit].clone());
                continue;
            }
            let q = replace_vowel(&q);
            if let Some(&hit) = vowels.get(&q) {
                ret.push(wordlist[hit].clone());
                continue;
            }
            ret.push("".to_owned());
        }
        ret
    }
}
