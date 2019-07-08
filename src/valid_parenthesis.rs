pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec!();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    v.push(c);
                    continue
                },
                _ => (),
            }
            if v.is_empty() {
                return false
            }
            let o = v.pop().unwrap();
            match (o, c) {
                ('(', ')') | ('[', ']') | ('{', '}') => (),
                _ => return false
            }
        }
        v.is_empty()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("()", true),
        ("()[]{}", true),
        ("(]", false),
        ("", true),
        ("([)]", false),
        ("{", false),
        ("{[]}", true),
    ];
    for (s, r) in cases {
        assert_eq!(Solution::is_valid(s.to_owned()), r, "{}", s);
    }
}
