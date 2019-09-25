pub struct Solution;

impl Solution {
    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        let row = board.len();
        if row < 1 {
            return 0
        }
        let col = board[0].len();
        let mut count = 0;
        for i in 0..row {
            for j in 0..col {
                if board[i][j] != 'X' {
                    continue;
                }
                count += 1;
                for k in j..col {
                    if board[i][k] == 'X' {
                        board[i][k] = '.';
                    } else {
                        break;
                    }
                }
                for k in (i+1)..row {
                    if board[k][j] == 'X' {
                        board[k][j] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
        count
    }
}
