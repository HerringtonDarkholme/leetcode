struct Solution {}

impl Solution {
    fn my_atoi(str: String) -> i32 {
        let chars = str.chars();
        let mut ret: i32 = 0;
        let mut sign = 1;
        let mut start = 0;
        for (i, c) in chars.enumerate() {
            match c {
                ' ' => continue,
                '+' => {
                    start = i + 1;
                    break;
                },
                '-' => {
                    sign = -1;
                    start = i + 1;
                    break;
                },
                '0'...'9' => {
                    start = i;
                    break;
                },
                _ => {
                    return 0
                }
            }
        }

        for c in str.get(start..str.len()).unwrap().chars() {
            match c {
                '0'...'9' => {
                    let num = ((c as u8) - ('0' as u8)) as i32;
                    if let Some(r1) = ret.checked_mul(10) {
                        if let Some(r2) = r1.checked_add(num * sign) {
                            ret = r2;
                            continue;
                        }
                    }
                    if sign > 0 {
                        return i32::max_value();
                    } else {
                        return i32::min_value();
                    }
                },
                _ => break,
            }
        }
        ret
    }
}

#[test]
fn test() {
    let tests = vec![
        ("123", 123),
        ("123wrap", 123),
        ("0123", 123),
        ("0000123", 123),
        ("   0000123", 123),
        ("-0123", -123),
        ("", 0),
        ("     -42", -42),
        ("+-+--2", 0),
        ("w101", 0),
        ("-91283472332", -2147483648),
        ("-2147483648", -2147483648),
        ("2147483648", 2147483647),
    ];
    for (case, expect) in tests {
        assert_eq!(expect, Solution::my_atoi(case.to_owned()));
    }
}
