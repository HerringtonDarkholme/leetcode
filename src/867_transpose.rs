impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut m = vec![vec![0; row]; col];
        for r in 0..row {
            for c in 0..col {
                m[c][r] = matrix[r][c];
            }
        }
        m
    }
}
