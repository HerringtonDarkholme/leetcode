pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let row = grid.len();
        let col = grid[0].len();
        let mut seen = HashSet::new();
        let mut num = 0;
        for r in 0..row {
            for c in 0..col {
                if seen.contains(&(r, c)) || grid[r][c] == '0' {
                    continue;
                }
                num += 1;
                seen.insert((r, c));
                let mut stack = vec![(r, c)];
                while !stack.is_empty() {
                    let (r, c) = stack.pop().unwrap();
                    let mut neighbors = vec![];
                    if r > 0 {
                        neighbors.push((r - 1, c));
                    }
                    if r < row - 1 {
                        neighbors.push((r + 1, c));
                    }
                    if c > 0 {
                        neighbors.push((r, c - 1));
                    }
                    if c < col - 1 {
                        neighbors.push((r, c + 1));
                    }
                    for neighbor in neighbors {
                        if seen.contains(&neighbor) || grid[neighbor.0][neighbor.1] == '0' {
                            continue;
                        }
                        seen.insert(neighbor);
                        stack.push(neighbor);
                    }
                }
            }
        }
        num
    }
}

/*
use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut seen = HashSet::new();
        let mut ret = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if dfs(&grid, r as i32, c as i32, &mut seen) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn dfs(grid: &Vec<Vec<char>>, r: i32, c: i32, seen: &mut HashSet<(i32, i32)>) -> bool {
    if seen.contains(&(r, c)) || grid[r as usize][c as usize] == '0' {
        return false;
    }
    seen.insert((r, c));
    for (x, y) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        let nr = r + x;
        let nc = c + y;
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            continue;
        }
        dfs(grid, nr, nc, seen);
    }
    true
}
*/
