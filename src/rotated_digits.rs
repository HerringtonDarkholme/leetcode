struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (0..=n).map(|mut c| {
            let mut some_no_reflex = false;
            while c > 0 {
                match c % 10 {
                    0 | 1 | 8 => (),
                    2 | 5 | 6| 9 => some_no_reflex = true,
                    _ => return 0,
                }
                c /= 10;
            }
            if some_no_reflex { 1 } else { 0 }
        }).sum()
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (10, 4),
        (10000, 2320),
    ], Solution::rotated_digits);
}
