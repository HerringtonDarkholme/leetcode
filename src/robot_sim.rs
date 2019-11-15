// leetcode 874
pub struct Solution;
use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;
        let mut obstacles_x = HashMap::new();
        let mut obstacles_y = HashMap::new();
        for ob in obstacles.iter() {
            obstacles_x.entry(ob[0]).or_insert_with(|| BTreeSet::new()).insert(ob[1]);
            obstacles_y.entry(ob[1]).or_insert_with(|| BTreeSet::new()).insert(ob[0]);
        }
        let mut max = 0;
        for command in commands {
            if command == -1 {
                dir = (dir + 1) % 4;
            } else if command == -2 {
                dir = (dir + 3) % 4;
            } else {
                match dir {
                    0 => {
                        y = obstacles_x.get(&x).and_then(|obs|
                            obs.range(y+1..=y+command).next()
                        ).map_or(y + command, |yy| yy - 1);
                    },
                    1 => {
                        x = obstacles_y.get(&y).and_then(|obs|
                            obs.range(x+1..=x+command).next()
                        ).map_or(x + command, |xx| xx - 1);
                    },
                    2 => {
                        y = obstacles_x.get(&x).and_then(|obs|
                            obs.range(y-command..y).rev().next()
                        ).map_or(y - command, |yy| yy + 1);
                    }
                    3 => {
                        x = obstacles_y.get(&y).and_then(|obs|
                            obs.range(x-command..x).rev().next()
                        ).map_or(x - command, |xx| xx + 1);
                    },
                    _ => (),
                }
                max = max.max(x * x + y * y);
            }
        }
        max
    }
}
