pub struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut r = 1;
        let mut s = a.clone();
        loop {
            if s.contains(&b) {
                return r
            }
            if s.len() > b.len() * 3 && r > 3 {
                return -1;
            }
            s += &a;
            r += 1;
        }
    }
}

#[test]
fn test() {
}
