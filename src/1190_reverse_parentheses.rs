impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut v = vec![vec![]];
        for c in s.chars() {
            if c == '(' {
                v.push(vec![]);
            } else if c == ')' {
                let mut r = v.pop().unwrap();
                r.reverse();
                v.last_mut().unwrap().extend(r);
            } else {
                v.last_mut().unwrap().push(c);
            }
        }
        v.pop().unwrap().into_iter().collect()
    }
}
