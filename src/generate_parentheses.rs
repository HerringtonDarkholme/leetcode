pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_owned()]
        }
        if n == 1 {
            return vec!["()".to_owned()]
        }
        let mut ret = vec![];
        for i in 0..n {
            for s1 in Solution::generate_parenthesis(i) {
                for s2 in Solution::generate_parenthesis(n - i - 1) {
                    ret.push(format!("({}){}", s1, s2));
                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    // too lazy...
}
