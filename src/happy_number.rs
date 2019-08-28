pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while !set.contains(&n) {
            if n == 1 {
                return true
            }
            set.insert(n);
            let mut sum = 0;
            while n > 0 {
                sum += (n % 10) * (n % 10);
                n /= 10;
            }
            n = sum;
        }
        false
    }
}
