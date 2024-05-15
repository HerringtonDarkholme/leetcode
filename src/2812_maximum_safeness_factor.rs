use std::collections::{BinaryHeap, HashSet};
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let mut distance = get_distance(grid);
        let mut heap = BinaryHeap::new();
        heap.push((distance[0][0], 0, 0));
        distance[0][0] = -1;
        while let Some((mut d, r, c)) = heap.pop() {
            if r + 1 == distance.len() && c + 1 == distance.len() {
                return d;
            }
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr as usize >= distance.len() || nc < 0 || nc as usize >= distance.len() || distance[nr as usize][nc as usize] < 0 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let nd = d.min(distance[nr][nc]);
                distance[nr][nc] = -1;
                heap.push((nd, nr, nc));
            }
        }
        -1
    }
}

fn get_distance(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut frontier = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                frontier.push((r, c));
            }
        }
    }
    let mut distance = vec![vec![-1; grid.len()]; grid.len()];
    let mut d = 0;
    while !frontier.is_empty() {
        let mut next = vec![];
        for (r, c) in frontier {
            if distance[r][c] >= 0 { continue; }
            distance[r][c] = d;
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr as usize >= grid.len() || nc < 0 || nc as usize >= grid.len() {
                    continue;
                }
                next.push((nr as usize, nc as usize));
            }
        }
        frontier = next;
        d += 1;
    }
    distance
}
