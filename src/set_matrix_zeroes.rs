pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut last_zero: Option<usize> = None;
        let rows = matrix.len();
        let cols = matrix[0].len();
        for i in 0..rows {
            let has_zero = matrix[i].iter().any(|&x| x == 0);
            if !has_zero {
                continue;
            }
            if let Some(last) = last_zero {
                for j in 0..cols {
                    if matrix[last][j] == 0 {
                        if matrix[i][j] != 0 {
                            matrix[i][j] = 0;
                        } else {
                            matrix[i][j] = 1;
                        }
                    } else {
                        matrix[i][j] = 1;
                    }
                }
            } else {
                for j in 0..cols {
                    if matrix[i][j] == 0 {
                        matrix[i][j] = 1;
                    } else {
                        matrix[i][j] = 0;
                    }
                }
            }
            last_zero = Some(i);
        }
        if let Some(last) = last_zero {
            for j in 0..cols {
                if matrix[last][j] == 1 {
                    for i in 0..rows {
                        matrix[i][j] = 0;
                    }
                }
            }
        }
    }
}
