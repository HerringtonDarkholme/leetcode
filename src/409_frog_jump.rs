pub struct Solution;
use std::collections::{HashSet, VecDeque};
/*
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut reachables = HashMap::new();
        let mut max = 0;
        for &stone in &stones {
            reachables.insert(stone, HashSet::new());
            max = stone;
        }
        reachables.get_mut(&0).unwrap().insert(0);
        let mut set = HashSet::new();
        for stone in stones {
            // f*ck rust
            std::mem::swap(&mut set, reachables.get_mut(&stone).unwrap());
            for &step in &set {
                for i in [1, 0, -1] {
                    let next = i + step;
                    if next <= 0 {
                        continue;
                    }
                    if let Some(new_stone) = reachables.get_mut(&(next + stone)) {
                        new_stone.insert(next);
                    }
                }
            }
            std::mem::swap(&mut set, reachables.get_mut(&stone).unwrap());
        }
        !reachables[&max].is_empty()
    }
}
// 0 1 3 5 6 8 12 17
// 0 1 2 2 3 2  4  5
//         1 3

*/


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
