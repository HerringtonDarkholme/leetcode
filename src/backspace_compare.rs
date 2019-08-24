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
/*
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s1 = Solution::resolve(s);
        let mut t1 = Solution::resolve(t);
        s1 == t1

    }
    fn resolve(s: String) -> Vec<char> {
        let mut chars: Vec<_> = s.chars().collect();
        let mut i = 0;
        for j in 0..chars.len() {
            if chars[j] == '#' {
                i = if i > 0 { i - 1} else { 0 };
            } else {
                chars[i] = chars[j];
                i += 1;
            }
        }
        chars.resize(i, ' ');
        chars
    }
}
 **/
