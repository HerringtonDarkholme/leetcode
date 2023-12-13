impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for r in 0..mat.len() {
            let mut found = 0;
            for c in 0..mat[r].len() {
                if mat[r][c] == 0 {
                    continue;
                }
                found += 1;
                if found > 1 {
                    break;
                }
                let mut cnt = 0;
                for i in 0..mat.len() {
                    if mat[i][c] != 0 && i != r {
                        found += 1;
                    }
                }
            }
            if found == 1 {
                ret += 1;
            }
        }
        ret
    }
}
