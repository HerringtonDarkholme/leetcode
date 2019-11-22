pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut ret = 0;
        let mut left = 0;
        for c in s.chars() {
            if c == '(' {
                left += 1;
            } else if left > 0 {
                left -= 1;
            } else {
                ret += 1;
            }
        }
        ret + left
    }
}
