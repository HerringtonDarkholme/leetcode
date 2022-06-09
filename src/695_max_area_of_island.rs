use std::collections::HashSet;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = HashSet::new();
        let mut max = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                max = dfs(&grid, r as i32, c as i32, &mut seen).max(max);
            }
        }
        max
    }
}

fn dfs(grid: &Vec<Vec<i32>>, r: i32, c: i32, seen: &mut HashSet<(i32, i32)>) -> i32 {
    if seen.contains(&(r, c)) || grid[r as usize][c as usize] == 0 {
        return 0;
    }
    let mut size = 1;
    seen.insert((r, c));
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + x;
            let nc = c + y;
            if nr < 0 || nr == grid.len() as i32 || nc < 0 || nc == grid[0].len() as i32 {
                continue;
            }
            size += dfs(grid, nr, nc, seen);
    }
    size
}
