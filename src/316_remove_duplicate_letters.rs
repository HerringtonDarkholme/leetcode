use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counter = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        let mut ret = vec![];
        for c in s.chars() {
            *counter.get_mut(&c).unwrap() -= 1;
            if ret.contains(&c) {
                continue;
            }
            while !ret.is_empty() && ret[ret.len() - 1] > c && counter[&ret[ret.len() - 1]] > 0 {
                ret.pop();
            }

            ret.push(c);
        }
        ret.into_iter().collect()
    }
}
