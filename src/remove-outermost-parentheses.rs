pub struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut index = 0;
        let mut ret = String::new();
        for c in s.chars() {
            if c == '(' {
                if index > 0 {
                    ret.push(c);
                }
                index += 1;
            } else if c == ')' {
                index -= 1;
                if index > 0 {
                    ret.push(c);
                }
            }
        }
        ret
    }
}
