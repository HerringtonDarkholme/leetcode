pub struct Solution;

use std::collections::{HashMap, HashSet};
type Map = HashMap<char, i32>;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.chars().collect();
        let t = t.chars().collect();
        min_window(s, t)
    }
}

fn min_window(s: Vec<char>, t: Vec<char>) -> String {
    if let Some((i, j)) = min_window_range(&s, &t) {
        s[i..=j].iter().collect()
    } else {
        "".to_string()
    }
}

fn build_map(t: &Vec<char>) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for &c in t.iter() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn min_window_range(s: &Vec<char>, t: &Vec<char>) -> Option<(usize, usize)> {
    let mut state = build_map(t);
    if let Some(p) = find_initial_range(s, &mut state) {
        Some(shrink_and_expand(s, &mut state, p))
    } else {
        None
    }
}

fn find_initial_range(s: &Vec<char>, state: &mut Map) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut start = -1;
    let mut set: HashSet<_> = state.keys().cloned().collect();
    while i < s.len() {
        if let Some(c) = state.get_mut(&s[i]) {
            *c -= 1;
            if start < 0 {
                start = i as i32;
            }
            if *c <= 0 {
                set.remove(&s[i]);
                if set.is_empty() {
                    return Some((start as usize, i))
                }
            }
        }
        i += 1;
    }
    None
}

fn shrink_and_expand(s: &Vec<char>, state: &mut Map, p: (usize, usize)) -> (usize, usize) {
    let mut ret = (p.0, p.1);
    let mut start = p.0;
    let mut end = p.1;
    let mut missing = '-';
    while end < s.len() {
        println!("before move staart {:?}", state);
        while start <= end {
            let opt = state.get_mut(&s[start]);
            start += 1;
            if opt.is_none() {
                continue;
            }
            let c = opt.unwrap();
            *c += 1;
            if *c <= 0 {
                continue;
            }
            if end - start + 1 < ret.1 - ret.0 {
                ret = (start - 1, end);
            }
            missing = s[start - 1];
            break;
        }
        println!("after move staart {:?}", state);
        end += 1;
        while end < s.len() {
            if let Some(c) = state.get_mut(&s[end]) {
                *c -= 1;
            }
            if s[end] == missing {
                if end - start < ret.1 - ret.0 {
                    ret = (start, end);
                }
                break;
            }
            end += 1;
        }
        println!("after move end {:?}", state);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        for ((s, t), a)in vec![
            (("ADOBECODEBANC", "ABC"), "BANC"),
            (("CINFJYEBKUJCHJKUFUYEWBJHBCHFBJFRHEWBRCCAAFFB", "CND"), ""),
            (("CINFJYEBKUJCHJKUFUYEWBJHBCHFBJFRHEWBRCCAAFFB", "CND"), ""),
            (("CJKRENUIMVCRNKJLRFDNKMJJNRSDFSDFVCDFSOHILVKUCJNKJFNE", "CMJILNRF"), "IMVCRNKJLRF"),
        ] {
            assert_eq!(Solution::min_window(s.into(), t.into()), String::from(a));
        }
    }
}


/*
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
*/
