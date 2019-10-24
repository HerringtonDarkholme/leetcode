pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false
        }
        let m = matrix.len();
        let n = matrix[0].len();
        if n == 0 {
            return false
        }
        let mut low = 0;
        let mut high = m*n - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            let r = mid / n;
            let c = mid % n;
            let e = matrix[r][c];
            if e < target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        matrix[low / n][low % n] == target
    }
}
