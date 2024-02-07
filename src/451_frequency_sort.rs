use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        for c in s.chars() {
            map.entry(c).or_insert_with(String::new).push(c);
        }
        let mut v: Vec<_> = map.into_values().collect();
        v.sort_by_key(|n| -(n.len() as i32));
        v.into_iter().collect()
    }
}
