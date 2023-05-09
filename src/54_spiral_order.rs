impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order(matrix)
    }
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut res = vec![];
    let (mut l, mut r, mut t, mut b) = (0, n - 1, 0, m - 1);
    while l <= r && t <= b {
        for i in l..=r {
            res.push(matrix[t][i]);
        }
        t += 1;
        for i in t..=b {
            res.push(matrix[i][r]);
        }
        if r == 0 {
            break;
        }
        r -= 1;
        if t <= b {
            for i in (l..=r).rev() {
                res.push(matrix[b][i]);
            }
            b -= 1;
        }
        if l <= r {
            for i in (t..=b).rev() {
                res.push(matrix[i][l]);
            }
            l += 1;
        }
    }
    res
}
