impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        max_square(matrix)
    }
}

fn max_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max = 0;
    let r = matrix.len();
    let c = matrix[0].len();
    // (cons_x, cons_y, square)
    let mut dp = vec![vec![(0,0,0); c]; r];
    let mut cnt = 0;
    for i in 0..r {
        if matrix[i][0] == '1' {
            cnt += 1;
            dp[i][0] = (cnt, 1, 1);
            max = 1;
        } else {
            cnt = 0;
        }
    }
    cnt = 0;
    for j in 0..c {
        if matrix[0][j] == '1' {
            cnt += 1;
            dp[0][j] = (cnt, 1, 1);
            max = 1;
        } else {
            cnt = 0;
        }
    }
    for i in 1..r {
        for j in 1..c {
            if matrix[i][j] == '0' {
                continue;
            }
            let cons_x = 1 + dp[i-1][j].0;
            let cons_y = 1 + dp[i][j-1].1;
            let max_square = (1+dp[i-1][j-1].2).min(cons_x).min(cons_y);
            max = max.max(max_square);
            dp[i][j] = (cons_x, cons_y, max_square);
        }
    }
    max * max
}
