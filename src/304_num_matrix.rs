struct NumMatrix {
    sum: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut sum = vec![vec![0; col + 1]; row + 1];
        let mut row_sum = 0;
        for r in 0..row {
            for c in 0..col {
                sum[r+1][c+1] = sum[r][c+1] + sum[r+1][c] - sum[r][c] + matrix[r][c];
            }
        }
        Self {
            sum,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let r2 = row2 as usize + 1;
        let c1 = col1 as usize;
        let c2 = col2 as usize + 1;
        let sum = &self.sum;
        sum[r2][c2] - sum[r1][c2] - sum[r2][c1] + sum[r1][c1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
