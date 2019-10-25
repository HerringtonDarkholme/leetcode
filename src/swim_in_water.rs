pub struct Solution;

use std::collections::{BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // make map
        let mut map = vec![(0, 0); n*n];
        for (row, v) in grid.iter().enumerate() {
            for (col, n) in v.iter().enumerate() {
                map[*n as usize] = (row as i32, col as i32);
            }
        }
        // maintain heap & visited
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(grid[0][0]));
        let mut visited = vec![false; n*n];
        visited[grid[0][0] as usize] = true;

        let mut max = 0;
        let n = n as i32;
        // bfs graph
        while !heap.is_empty() {
            let pos = heap.pop().unwrap().0;
            let (r, c) = map[pos as usize];
            max = max.max(pos);
            if r == n - 1 && c == n - 1 {
                break;
            }
            for (x, y) in vec![(r, c+1), (r, c-1), (r+1, c), (r-1,c)] {
                if x >= 0 && y >= 0 && x < n && y < n {
                    let e = grid[x as usize][y as usize];
                    if visited[e as usize] {
                        continue;
                    }
                    heap.push(Reverse(e));
                    visited[e as usize] = true;
                }
            }
        }
        max
    }
}
