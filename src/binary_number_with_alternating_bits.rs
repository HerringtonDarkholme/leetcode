// leetcode 693
pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        // 10101010            1010101
        //  1010101              10101            10101
        (n & (n >> 1)) == 0 && (n & (n >> 2)) == (n >> 2)
    }
}
