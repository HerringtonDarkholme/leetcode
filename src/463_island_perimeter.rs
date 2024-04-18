impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 0 { continue; }
                for (dr, dc) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
                        ret += 1;
                    } else if grid[nr as usize][nc as usize] == 0 {
                        ret += 1;
                    }
                }
            }
        }
        ret
    }
}
