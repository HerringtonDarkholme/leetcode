impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] > 0 {
                    ret = dfs(r, c, &mut grid).max(ret);
                }
            }
        }
        ret
    }
}
fn dfs(r: usize, c: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut origin = grid[r][c];
    let mut max = 0;
    grid[r][c] = 0;
    for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nr = r as i32 + x;
        let nc = c as i32 + y;
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            continue;
        }
        if grid[nr as usize][nc as usize] == 0 {
            continue;
        }
        max = dfs(nr as usize, nc as usize, grid).max(max);
    }
    grid[r][c] = origin;
    origin + max
} 
