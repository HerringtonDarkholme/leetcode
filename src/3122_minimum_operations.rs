impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let counter = count_nums(grid);
        let mut dp = [0; 10];
        for c in 0..cols {
            let mut next = [0; 10];
            for i in 0..10 {
                let mut min = i32::MAX;
                for j in 0..10 {
                    if i == j { continue; }
                    min = min.min(dp[j]);
                }
                next[i] = cost(rows, c, i, &counter) + min;
            }
            dp = next;
        }
        dp.into_iter().min().unwrap()
    }
}
fn count_nums(grid: Vec<Vec<i32>>) -> Vec<[i32; 10]> {
    let mut ret = vec![[0; 10]; grid[0].len()];
    for c in 0..grid[0].len() {
        for r in 0..grid.len() {
            let num = grid[r][c] as usize;
            ret[c][num] += 1;
        }
    }
    ret
}
fn cost(grid_len: usize, c: usize, n: usize, counter: &Vec<[i32; 10]>) -> i32 {
    grid_len as i32 - counter[c][n]
}
// cost(c, n)
/* dp(c, n) = min(
    min(dp(c - 1, m) for m in 0-9 if m != n) 
    + cost(c, n) 
    for n in 0-9
)
*/
