pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let c = ((1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 2.0) as i32;
        c - 1
    }
}
