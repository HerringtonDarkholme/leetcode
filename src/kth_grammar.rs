struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        (k - 1).count_ones() as i32 % 2
        // let mut k = k - 1;
        // let mut r = 0;
        // while k > 0 {
        //     r ^= k % 2;
        //     k /= 2;
        // }
        // r
        // consider zero based k, if k is even, it is same as symbol at k/2
        // else, it is xor of symbol at k/2. converge util 0.
        // above is a translation of recursion
        // fn aux(k: i32) -> i32 { if k == 0 {} else { aux(k/2) ^ (k % 2)} }
        // aux(k -1)
    }
}
