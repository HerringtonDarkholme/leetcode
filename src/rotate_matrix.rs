impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut i = 0;
        // rotate matrix one layer by one layer
        while 2 * i < n {
            for c in i..n-i-1 {
                let mut row = i;
                let mut col = c;
                let mut last = matrix[row][col];
                let mut next_row = 0;
                let mut next_col = 0;
                for _ in 0..4 {
                    next_row = col;
                    next_col = n - 1 - row;
                    let next = matrix[next_row][next_col];
                    matrix[next_row][next_col] = last;
                    last = next;
                    row = next_row;
                    col = next_col;
                }
            }
            i += 1;
        }
    }
}
