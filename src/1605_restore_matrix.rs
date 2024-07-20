impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; col_sum.len()]; row_sum.len()];
        while aux(&mut row_sum, &mut col_sum, &mut ret) {}
        ret
    }
}

fn aux(rows: &mut [i32], cols: &mut [i32], ret: &mut Vec<Vec<i32>>) -> bool {
    let mut row = 0;
    let mut col = 0;
    for i in 1..rows.len() {
        if rows[i] > 0 && (rows[row] == 0 || rows[i] < rows[row]) {
            row = i;
        }
    }
    for i in 1..cols.len() {
        if cols[i] > 0 && (cols[col] == 0 || cols[col] < cols[col]) {
            col = i;
        }
    }
    if rows[row] > 0 || cols[col] > 0 {
        let x = if rows[row] == 0 {
            cols[col]
        } else {
            rows[row].min(cols[col])
        };
        ret[row][col] = x;
        rows[row] -= x;
        cols[col] -= x;
        true
    } else {
        false
    }
}
