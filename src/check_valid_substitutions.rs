pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            if c == 'a' || c == 'b' {
                stack.push(c);
                continue;
            }
            let b = stack.pop().filter(|c| c == 'b');
            let a = stack.pop().filter(|c| c == 'a');
            if a.is_none() || b.is_none() {
                return false
            }
        }
        stack.is_empty()
    }
}
