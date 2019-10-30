pub struct Solution;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
        if grid.is_empty() || grid[0].is_empty() {
            return grid
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        dfs(&mut grid,  r0, c0, color, &mut visited);
        grid
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, color: i32, visited: &mut Vec<Vec<bool>>) {
    let rr = r as usize;
    let cc = c as usize;
    visited[rr][cc] = true;
    let origin = grid[rr][cc];
    let rmax = grid.len() as i32;
    let cmax = grid[0].len() as i32;
    for (i, j) in vec![(0,1), (0, -1), (1, 0), (-1, 0)] {
        if r + i >= rmax || r + i < 0 || c + j >= cmax || c + j < 0 {
            grid[rr][cc] = color;
            continue;
        }
        let rn = (r + i) as usize;
        let cn = (c + j) as usize;
        if visited[rn][cn] {
            continue;
        }
        if grid[rn][cn] == origin {
            dfs(grid, rn as i32, cn as i32, color, visited);
        } else {
            grid[rr][cc] = color;
        }
    }
}
