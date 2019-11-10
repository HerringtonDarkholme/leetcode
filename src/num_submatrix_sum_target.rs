pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut prefix_sum = vec![vec![0; col+1]; row];
        for r in 0..row {
            for c in 0..col {
                prefix_sum[r][c+1] = prefix_sum[r][c] + matrix[r][c];
            }
        }
        let mut ret = 0;
        for i in 0..col {
            for j in (i+1)..=col {
                let mut map = HashMap::new();
                map.insert(0, 1);
                let mut cur = 0;
                for k in 0..row {
                    let row_sum = prefix_sum[k][j] - prefix_sum[k][i];
                    cur += row_sum;
                    ret += *map.get(&(cur - target)).unwrap_or(&0);
                    *map.entry(cur).or_insert(0) += 1;
                }
            }
        }
        ret
    }
}
