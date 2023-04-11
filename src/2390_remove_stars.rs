impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if c == '*' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}
