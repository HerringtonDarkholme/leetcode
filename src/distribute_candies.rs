pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let divide = candies.len() / 2;
        let kinds = candies.iter().collect::<HashSet<_>>().len();
        divide.min(kinds) as i32
    }
}
