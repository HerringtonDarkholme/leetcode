impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut paths = vec![0; grid[0].len() + 1];
        paths[1] = 1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                paths[j + 1] += paths[j];
                paths[j + 1] *= 1 - grid[i][j];
            }
        }
        paths[paths.len() - 1]
    }
}

/*pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row = obstacle_grid.len();
        if row == 0 {
            return 0
        }
        let col = obstacle_grid[0].len();
        let mut paths = vec![vec![0; col]; row];
        paths[0][0] = 1 - obstacle_grid[0][0];
        for j in 1..col {
            if obstacle_grid[0][j] == 0 {
                paths[0][j] = paths[0][j - 1];
            }
        }
        for i in 1..row  {
            if obstacle_grid[i][0] == 0 {
                paths[i][0] = paths[i - 1][0];
            }
            for j in 1..col {
                if obstacle_grid[i][j] == 0 {
                    paths[i][j] = paths[i -1][j] + paths[i][j -1];
                }
            }
        }
        paths[row - 1][col - 1]
    }
}
*/
