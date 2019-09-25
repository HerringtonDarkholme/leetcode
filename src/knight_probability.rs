pub struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let n = n as usize;
        let r = r as usize;
        let c = c as usize;
        let mut board = vec![0f64; n*n];
        board[r*n + c] = 1.0;
        let moves = vec![
            (2, 1),
            (-2, 1),
            (2, -1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),

        ];
        let n = n as i32;
        let mut p = 1f64;
        for _ in 0..k {
            let mut next = vec![0f64; n as usize *n as usize];
            for i in 0..n*n {
                let row = i / n;
                let col = i % n;
                for &m in moves.iter() {
                    let nr = row + m.0;
                    let nc = col + m.1;
                    if nr < 0 || nr >= n || nc < 0 || nc >= n {
                        continue;
                    } else {
                        let ni = nr * n + nc;
                        next[ni as usize] += board[i as usize] / 8.0;
                    }
                }
            }
            board = next;
        }
        board.iter().sum()
    }
}
