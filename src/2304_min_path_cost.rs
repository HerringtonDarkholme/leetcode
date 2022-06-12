use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;
impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        for j in 0..grid[0].len() {
            heap.push((Reverse(grid[0][j]), 0, j));
        }
        while let Some((Reverse(cost), i, j)) = heap.pop() {
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            if i == grid.len() - 1 {
                return cost;
            }
            let num = grid[i][j] as usize;
            for nj in 0..move_cost[num].len() {
                let ni = i + 1;
                let new_cost = cost + move_cost[num][nj] + grid[ni][nj];
                heap.push((Reverse(new_cost), ni, nj));
            }
        }
        -1
    }
}

