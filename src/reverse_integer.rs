struct Solution {}

impl Solution {
    fn reverse(x: i32) -> i32 {
        if x == i32::min_value() {
            return 0
        }
        if x < 0 {
            return -Solution::reverse(-x)
        }
        let mut x = x;
        let mut y = 0;
        let max = i32::max_value();
        while x != 0 {
            if y > max / 10 {
                return 0
            }

            y = y * 10;
            if y > max - x % 10 {
                return 0
            }
            y = y + x % 10;
            x /= 10
        }
        y
    }
}

#[test]
fn test() {
    let tests = vec![
        (123, 321),
        (2147483647, 0),
        (2147483641, 1463847412),
        (-2147483648, 0),
        (1463847422, 0),
        (1463847412, 2147483641),
        (120, 21),
        (-123, -321),
    ];
    for (test, expect) in tests {
        assert_eq!(expect, Solution::reverse(test));
    }
}
