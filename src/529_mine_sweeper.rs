impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        update(&mut board, click[0] as usize, click[1] as usize);
        board
    }
}
fn update(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let c = board[x][y];
    if c == 'M' {
        board[x][y] = 'X';
    } else if c == 'E' {
        let mut mines = 0;
        for dx in vec![0, 1, -1] {
            for dy in vec![0, 1, -1] {
                if dx == 0 && dy == 0 || x == 0 && dx < 0 || x == board.len() - 1 && dx > 0 || y == 0 && dy < 0 || y == board[0].len() - 1 && dy > 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if board[nx as usize][ny as usize] == 'M' {
                    mines += 1;
                }
            }
        }
        if mines == 0 {
            board[x][y] = 'B';
            for dx in vec![0, 1, -1] {
                for dy in vec![0, 1, -1] {
                    if dx == 0 && dy == 0 || x == 0 && dx < 0 || x == board.len() - 1 && dx > 0 || y == 0 && dy < 0 || y == board[0].len() - 1 && dy > 0 {
                        continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                update(board, nx as usize, ny as usize);    
            }
        }
        } else {
            board[x][y] = ('0' as u8 + mines) as char;
        }
    }
}
