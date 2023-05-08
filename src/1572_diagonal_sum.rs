impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let len = mat.len();
        for (i, row) in mat.iter().enumerate() {
            sum += row[i];
            sum += row[len - i - 1];
        }
        if len % 2 == 1 {
            sum -= mat[len / 2][len / 2];
        }
        sum
    }
}
