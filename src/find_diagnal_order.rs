// leetcode 498
pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![];
        let row = matrix.len();
        if row == 0 {
            return ret
        }
        let col = matrix[0].len();
        if col == 0 {
            return ret
        }
        let mut r = 0;
        let mut c = 0;
        while !(r == row - 1 && c == col - 1) {
            ret.push(matrix[r][c]);
            if (r + c) % 2 == 0 {
                // go up
                if c == col - 1 {
                    r += 1;
                    continue;
                }
                c += 1;
                if r != 0 {
                    r -= 1;
                }
            } else {
                // go down
                if r == row - 1 {
                    c += 1;
                    continue;
                }
                r += 1;
                if c != 0 {
                    c -= 1;
                }
            }
        }
        ret.push(matrix[r][c]);
        ret
    }
}
