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
                if board[i][j] == 'X' && (
                    i == 0 || board[i-1][j] != 'X'
                ) && (
                    j == 0 || board[i][j-1] != 'X'
                ) {
                    count += 1;
                }
            }
        }
        count
    }
}
