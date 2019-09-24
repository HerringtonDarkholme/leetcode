pub struct Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap();
        let mut stones: HashSet<i32> = stones.into_iter().collect();
        if !stones.contains(&1) {
            return false
        }
        if stones.len() == 2 {
            return true
        }
        let mut set = HashSet::new();
        let mut stack = VecDeque::new();
        stack.push_back((1, 1));
        set.insert((1, 1));
        while !stack.is_empty() {
            let (pos, step) = stack.pop_front().unwrap();
            for i in vec![-1, 0 , 1] {
                if step + i > 0 {
                    let k = (pos + step + i, step + i);
                    if k.0 == target {
                        return true
                    }
                    if stones.contains(&k.0) && !set.contains(&k) {
                        stack.push_back(k);
                        set.insert(k);
                    }
                }
            }
        }
        false
    }
}
