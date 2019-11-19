// leetcode 781
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut map = HashMap::new();
        for ans in answers {
            *map.entry(ans).or_insert(0) += 1;
            if map[&ans] > ans {
                max += ans + 1;
                map.remove(&ans);
            }
        }
        max + map.keys().map(|c| c + 1).sum::<i32>()
    }
}
