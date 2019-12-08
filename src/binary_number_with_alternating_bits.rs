// leetcode 693
pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        if n & 1 == 0 {
            aux(n)
        } else {
            aux(n >> 1)
        }
    }
}

fn aux(mut n: i32) -> bool {
    while n != 0 {
        if (n & 3) ^ 2 != 0 {
            return false
        }
        n >>= 2
    }
    true
}
