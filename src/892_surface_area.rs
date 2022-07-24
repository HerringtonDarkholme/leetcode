impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut area = 0;
        for r in 0..row {
            for c in 0..col {
                let curr = grid[r][c];
                if curr == 0 {
                    area -= 2;
                }
                let up = if r == 0 { 0 } else { grid[r - 1][c] };
                let down = if r == row - 1 { 0 } else { grid[r + 1][c] };
                let left = if c == 0 { 0 } else { grid[r][c - 1] };
                let right = if c == col - 1 { 0 } else { grid[r][c + 1] };
                area += 0.max(curr - up) + 0.max(curr - down) + 0.max(curr - left) + 0.max(curr - right);
            }
        }
        area + 2 * (row * col) as i32
    }
}

// 9 + 9 + 3 * 4 + 4
//
// 1 2, 3 4, 1 3
