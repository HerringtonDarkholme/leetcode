pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Solution::pow(x, n as i64)
    }
    pub fn pow(x: f64, mut n: i64) -> f64 {
        if n < 0 {
            1.0 / Solution::pow(x, -n)
        } else {
            let mut x = x;
            let mut r = 1.0;
            while n > 0 {
                if n & 1 == 1 {
                    r *= x;
                }
                n = n >> 1;
                x *= x;
            }
            r
        }
    }
}
