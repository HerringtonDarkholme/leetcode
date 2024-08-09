impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if is_magic(r, c, &grid) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn is_magic(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
    if r + 3 > grid.len() || c + 3 > grid[0].len() {
        return false;
    }
    let sum = grid[r][c] + grid[r][c+1] + grid[r][c+2];
    let offsets = [
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];
    for offset in offsets {
        let s = offset.into_iter().map(|(dr, dc)| grid[r+dr][c+dc]).sum::<i32>();
        if sum != s {
            return false;
        }
    }
    let mut nums = vec![false;9];
    for i in 0..3 {
        for j in 0..3 {
            let index = grid[r+i][c+j] - 1;
            if index < 0 || index >= 9 { return false; }
            nums[index as usize] = true;
        }
    }
    nums.iter().all(|n| *n)
}
