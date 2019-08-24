pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s1 = Solution::resolved(s);
        let mut t1 = Solution::resolved(t);
        s1 == t1

    }
    fn resolved(s: String) -> String {
        let mut s1 = String::new();
        for c in s.chars() {
            if c == '#' {
                s1.pop();
            } else {
                s1.push(c);
            }
        }
        s1
    }
}
