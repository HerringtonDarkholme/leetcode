pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut left = 0;
        let t = build_map(t);
        let s = s.chars().collect::<Vec<_>>();
        let mut ss = HashMap::new();
        let mut min = (0, usize::max_value());
        for i in 0..s.len() {
            let c = s[i];
            *ss.entry(c).or_insert(0) += 1;
            while contains(&ss, &t) {
                if min.1 - min.0 > i - left {
                    min.1 = i;
                    min.0 = left;
                }
                *ss.get_mut(&(s[left])).unwrap() -= 1;
                left += 1;
            }
        }
        if min.1 == usize::max_value() {
            "".to_owned()
        } else {
            s[min.0..=min.1].iter().collect()
        }
    }
}
fn build_map(t: String) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in t.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn contains(s: &HashMap<char, usize>, t: &HashMap<char, usize>) -> bool {
    for (k, v) in t {
        let n = s.get(k).unwrap_or(&0);
        if *n < *v {
            return false
        }
    }
    true
}
