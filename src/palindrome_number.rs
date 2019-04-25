struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        if x == 0 {
            return true
        }
        let mut i = 1_000_000_000;
        while x / i == 0 {
            i = i / 10;
        }
        let mut j = 1;
        while i > j {
            if (x / i) % 10 != (x / j) % 10 {
                return false
            }
            i = i / 10;
            j = j * 10;
        }
        true
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (121, true),
        (-121, false),
        (123, false),
        (10, false),
        (0, true),
        (1, true),
        (2, true),
        (999, true),
        (899, false),
        (9899, false),
        (2112, true),
        (21012, true),
        (2100000012, true),
        (2147483647, false),
        (i32::max_value(), false),
    ], Solution::is_palindrome);
}
