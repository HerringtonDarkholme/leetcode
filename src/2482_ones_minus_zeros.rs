impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ones_row = vec![];
        let mut ones_col = vec![];
        for r in &grid {
            let mut ones = 0;
            for c in r {
                if *c == 1 {
                    ones += 1;
                }
            }
            ones_row.push(ones);
        }
        for c in 0..grid[0].len() {
            let mut ones = 0;
            for r in 0..grid.len() {
                if grid[r][c] == 1 {
                    ones += 1;
                }
            }
            ones_col.push(ones);
        }
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                grid[r][c] = 
                  2 * ones_row[r] + 2 * ones_col[c] - (grid.len() + grid[0].len()) as i32;
            }
        }
        grid
    }
}
