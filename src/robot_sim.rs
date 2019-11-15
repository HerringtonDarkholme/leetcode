pub struct Solution;
use std::collections::{HashSet};

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;
        let obstacles: HashSet<_> = obstacles
            .iter().map(|v| (v[0], v[1]))
            .collect();
        let moves = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ];
        let mut max = 0;
        for mut command in commands {
            if command == -1 {
                dir = (dir + 1) % 4;
            } else if command == -2 {
                dir = (dir + 3) % 4;
            } else {
                let (i, j) = moves[dir];
                while command > 0 && !obstacles.contains(&(x + i, y + j)) {
                    x += i;
                    y += j;
                    command -= 1;
                }
                max = max.max(x * x + y * y);
            }
        }
        max
    }
}
