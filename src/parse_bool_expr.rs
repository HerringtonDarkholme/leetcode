pub struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let (r, i) = Solution::parse_worker(&expression.chars().collect::<Vec<_>>(), 0);
        assert_eq!(i, expression.len());
        r
    }

    fn parse_worker(chars: &[char], i: usize) -> (bool, usize) {
        match chars[i] {
            't'  => (true, i+1),
            'f' => (false, i+1),
            '!' => {
                assert_eq!(chars[i+1], '(');
                let (ret, new_i) = Solution::parse_worker(chars, i+2);
                assert_eq!(chars[new_i], ')');
                (!ret, new_i + 1)
            },
            '&' => {
                assert_eq!(chars[i+1], '(');
                let mut ret = true;
                let mut new_i = i + 2;
                loop {
                    let (r, ii) = Solution::parse_worker(chars, new_i);
                    ret &= r;
                    new_i = ii + 1;
                    if chars[ii] == ')' {
                        break;
                    }
                    assert_eq!(chars[ii], ',');
                }
                (ret, new_i)
            },
            '|' => {
                assert_eq!(chars[i+1], '(');
                let mut ret = false;
                let mut new_i = i + 2;
                loop {
                    let (r, ii) = Solution::parse_worker(chars, new_i);
                    ret |= r;
                    new_i = ii + 1;
                    if chars[ii] == ')' {
                        break;
                    }
                    assert_eq!(chars[ii], ',');
                }
                (ret, new_i)
            },
            _ => {
                panic!("impossible!")
            },
        }
    }
}


#[test]
fn test() {
    for i in vec![
        "!(f)",
        "&(t,f)",
        "|(&(t,f,t),!(t))",
        "|(t,f)",
        "|(t,t)",
        "|(t,t,t,t,f,f,f)",
        "&(t,t,t,t,f,f,f)",
        "|(&(t,t,t,t,f,f,f),!(&(t,f)))",
    ] {
        Solution::parse_bool_expr(i.to_string());
    }
}
