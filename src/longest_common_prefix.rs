struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }
        let mut ret = &strs[0][..];
        let mut merge = |s: &str| {
            let mut i = 0;
            for (c1, c2) in ret.chars().zip(s.chars()) {
                if c1 != c2 {
                    break;
                }
                i += 1;
            }
            ret = &ret[0..i];
        };
        for s in &strs[1..] {
            merge(s);
        }
        ret.to_owned()
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (vec_str!["flower","flow","flight"], "fl".to_owned()),
        (vec_str!["flower","flow","flowing"], "flow".to_owned()),
        (vec_str![""], "".to_owned()),
        (vec_str!["dog","racecar","car"], "".to_owned()),
        (vec_str!["aa","a"], "a".to_owned()),
        (vec_str![], "".to_owned()),
    ], Solution::longest_common_prefix);
}
