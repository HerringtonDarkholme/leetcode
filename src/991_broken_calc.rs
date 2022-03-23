use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut ans = 0;
        // work backward, while target is greater than start
        // since we can only pick either +1 or /2
        while start_value < target {
            ans += 1;
            if target % 2 == 1 {
                target += 1;
            } else {
                target /= 2;
            }
        }
        ans + start_value - target
    }
}
