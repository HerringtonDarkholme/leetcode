use std::collections::HashMap;
use std::cmp;

struct Solution {}

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut start = 0;
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(&last) = map.get(&c) {
                if last >= start {
                    // update current start
                    if i - start > max {
                        max = i - start;
                    }
                    start = last + 1;
                }
            }
            map.insert(c, i);
        }
        max = cmp::max(s.len() - start, max);
        max as i32
    }
}

#[test]
fn test() {
    let tests = vec!("aaaaa", "abcabcbb", "abcde", "pwwkew", "vcbnujpsdn");
    for test in tests {
        println!("{} {}", test, Solution::length_of_longest_substring(test.into()));
    }
}

/*
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = [-1; 128];
        let mut start = 0;
        let mut max = 0;
        for (i, c) in s.bytes().enumerate() {
            let i = i as i32;
            let c = c as usize;
            let j = seen[c];
            if j >= start {
                start = j + 1;
            }
            seen[c] =  i;
            max = max.max(i - start + 1);
        }
        max as i32
    }
}
*/
