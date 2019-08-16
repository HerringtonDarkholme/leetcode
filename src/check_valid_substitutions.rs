pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            if c == 'a' || c == 'b' {
                stack.push(c);
            }
            let b = stack.pop();
            let a = stack.pop();
            if a.is_none() || b.is_none() {
                return false
            }
            let is_b = b.unwrap() == 'b';
            let is_a = a.unwrap() == 'a';
            if !is_b || !is_a {
                return false
            }

        }
        stack.is_empty()
    }
}
