impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![];
        for r in 0..matrix.len() {
            let mut min = 0;
            for c in 1..matrix[0].len() {
                if matrix[r][c] < matrix[r][min] {
                    min = c;
                }
            }
            let mut is_valid = true;
            for rr in 0..matrix.len() {
                if matrix[rr][min] > matrix[r][min] {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                ret.push(matrix[r][min]);
            }
        }
        ret
    }
}
