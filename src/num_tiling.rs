// leetcode 790
pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(mut n: i32) -> i32 {
        if n < 2 {
            return 1
        }
        if n == 2 {
            return 2
        }

        let mut n_1 = 2;
        let mut n_2 = 1;
        let mut n_sum = 1;
        while n > 2 {
            let next = (n_1 + n_2) % MOD + (2 * n_sum) % MOD;
            n_sum = (n_sum + n_2) % MOD;
            n_2 = n_1;
            n_1 = next % MOD;
            n -= 1;
        }
        n_1
    }
 }
