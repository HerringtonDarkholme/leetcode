pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1
        }
        let len = grid.len() as i32;
        let mut stack = VecDeque::new();
        stack.push_back(0);
        let mut visited = vec![0; len as usize * len as usize];
        visited[0] = 1;
        while !stack.is_empty() {
            let pos = stack.pop_front().unwrap();
            let trip = visited[pos as usize];
            if pos == len * len - 1 {
                return trip
            }
            let row = pos / len;
            let col = pos % len;
            for &i in [-1, 0, 1].iter() {
                if row == 0 && i == -1 || row + i == len {
                    continue;
                }
                let r = row + i;
                for &j in [-1, 0, 1].iter() {
                    if col == 0 && j == -1 || col + j == len {
                        continue;
                    }
                    let c = col + j;
                    let p = (r * len + c) as usize;
                    if visited[p] != 0 || grid[r as usize][c as usize] == 1 {
                        continue;
                    }
                    visited[p] = trip + 1;
                    stack.push_back(p as i32);
                }
            }
        }
        -1
    }
}
