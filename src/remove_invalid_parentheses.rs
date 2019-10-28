pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let (left, right) = count_lr(&s);
        let mut set = HashSet::new();
        let s = &s.chars().collect::<Vec<_>>();
        assemble_str(String::new(), s, 0, left, right, &mut set);
        set.into_iter().collect()
    }
}

fn assemble_str(mut acc: String, s: &[char], mut existing: usize, left: usize, right: usize, set: &mut HashSet<String>) {
    if left == 0 && right == 0 {
        for &c in s {
            if existing == 0 && c == ')' {
                return
            }
            existing = match c {
                '(' => existing + 1,
                ')' => existing - 1,
                _ => existing,
            };
            acc.push(c);
        }
        set.insert(acc);
        return
    }
    if s.is_empty() {
        return
    }
    if right > 0  && s[0] == ')' {
        assemble_str(acc.clone(), &s[1..], existing, left, right - 1, set);
    } else if right == 0 && s[0] == '(' {
        assemble_str(acc.clone(), &s[1..], existing, left - 1, right, set);
    }
    if s[0] != ')' || existing > 0 {
        acc.push(s[0]);
        let l = match s[0] {
            '(' => existing + 1,
            ')' => existing - 1,
            _ => existing,
        };
        assemble_str(acc, &s[1..], l ,left, right, set);
    }

}

fn count_lr(s: &String) -> (usize, usize) {
    let mut left = 0;
    let mut right = 0;
    for c in s.chars() {
        if c == '(' {
            left += 1;
        } else if c == ')' {
            if left > 0 {
                left -= 1;
            } else {
                right += 1;
            }
        }
    }
    (left, right)
}
