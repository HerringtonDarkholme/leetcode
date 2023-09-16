use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
        let mut frontier = BinaryHeap::new();
        frontier.push((0, 0, 0)); // effor, row, col
        let row = heights.len() as i32;
        let col = heights[0].len() as i32;
        while let Some((e, r, c)) = frontier.pop() {
            if r == row - 1 && c == col - 1 {
                return -e
            }
            if visited[r as usize][c as usize] {
                continue;
            }
            visited[r as usize][c as usize] = true;
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= row || nc < 0 || nc >= col || visited[nr as usize][nc as usize] {
                    continue;
                }
                let ne = (heights[r as usize][c as usize] - heights[nr as usize][nc as usize]).abs().max(-e);

                frontier.push((-ne, nr, nc));
            }
        }
        -1 // impossible
    }
}
