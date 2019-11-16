// leetcode 1036
use std::collections::HashSet;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        const MAX: i32 = 1_000_000;
        let size = blocked.len() as i32;

        let blocked: HashSet<_> = blocked
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect();
        let is_blocked = |x, y| {
            if x < 0 || y < 0 || x == MAX || y == MAX {
                true
            } else {
                blocked.contains(&(x, y))
            }
        };
        let manhattan_dist = |(x1, y1): (i32, i32), (x2, y2): (i32, i32)| {
            ((x1 - x2).abs() + (y1 -y2).abs()) as i32
        };
        let surrounded = |src, dst| {
            let mut stack = vec![src];
            let mut visited = HashSet::new();
            visited.insert(src);
            while !stack.is_empty() {
                let curr = stack.pop().unwrap();
                let dist = manhattan_dist(src, curr);
                if dist >= size || curr == dst {
                    return false
                }
                for (i, j) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let next = (curr.0 + i, curr.1 + j);
                    if !is_blocked(next.0, next.1) && !visited.contains(&next) {
                        visited.insert(next);
                        stack.push(next);
                    }
                }
            }
            true
        };
        !surrounded((source[0], source[1]), (target[0], target[1])) && !surrounded((target[0], target[1]), (source[0], source[1]))
    }
}

