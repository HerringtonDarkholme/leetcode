pub struct Solution;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut r = 1;
        let mut k = 2;
        while k * (k + 1) <= n * 2 {
            if n * 2 % k == 0 && (2 * n / k) % 2 != k % 2 {
                r += 1;
            }
            k += 1;
        }
        r
    }
}
