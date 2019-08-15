pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if m == i32::max_value() {
            return m
        }
        let mut ret = m;
        for i in m+1..=n {
            ret &= i;
            if ret == 0 {
                return ret
            }
        }
        ret
    }
}
