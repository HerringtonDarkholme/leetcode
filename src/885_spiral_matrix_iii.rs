impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut cnt = rows * cols;
        let mut ret = vec![];
        let (mut r, mut c, mut dir) = (r_start, c_start, 1);
        let (mut step, mut max) = (0, 1);
        while cnt > 0 {
            if r >= 0 && r < rows && c >= 0 && c < cols {
                cnt -= 1;
                ret.push(vec![r, c]);
            }
            if dir == 1 {
                c += 1;
                step += 1;
                if step == max {
                    step = 0;
                    dir = 2;
                }
            } else if dir == 2 {
                r += 1;
                step += 1;
                if step == max {
                    step = 0;
                    max += 1;
                    dir = 3;
                }
            } else if dir == 3 {
                c -= 1;
                step += 1;
                if step == max {
                    step = 0;
                    dir = 4;
                }
            } else {
                r -= 1;
                step += 1;
                if step == max {
                    step = 0;
                    max += 1;
                    dir = 1;
                }
            }
        }
        ret
    }
}
