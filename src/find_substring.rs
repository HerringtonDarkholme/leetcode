pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() {
            return vec![]
        }
        let step = words[0].len();
        if step == 0 {
            return (0..=s.len()).map(|c| c as i32).collect()
        }
        let mut words_map = HashMap::new();
        let mut total_count = 0;
        for w in words {
            *words_map.entry(w).or_insert(0) += 1;
            total_count += 1;
        }
        let mut ret = vec![];
        for offset in 0..step {
            let mut start = offset;
            let mut end = offset;
            let mut map = words_map.clone();
            let mut count = total_count;
            while end + step <= s.len() {
                let word = &s[end..end+step];
                while start < end && *map.get(word).unwrap_or(&0) <= 0 {
                    let pre_word = &s[start..start+step];
                    *map.get_mut(pre_word).unwrap() += 1;
                    count += 1;
                    start += step;
                }
                if map.contains_key(word) {
                    *map.get_mut(word).unwrap() -= 1;
                    count -= 1;
                } else {
                    start += step;
                }
                end += step;
                if count == 0 {
                    ret.push(start as i32);
                }
            }
        }
        ret
    }
}
