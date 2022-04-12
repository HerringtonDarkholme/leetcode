impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let row = board.len();
        let col = board[0].len();
        for r in 0..row {
            for c in 0..col {
                let neighbors = find_neighbors(r as i32, c as i32, board);
                board[r][c] |= (neighbors << 1);
            }
        }
        for r in 0..row {
            for c in 0..col {
                let neighbors = board[r][c] >> 1;
                let val = board[r][c] & 1;
                board[r][c] = can_live(val, neighbors);
            }
        }
    }
}

fn find_neighbors(r: i32, c: i32, board: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let row = board.len() as i32;
    let col = board[0].len() as i32;
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            let nr = r + i;
            let nc = c + j;
            if nr < 0 || nc < 0 || nr >= row || nc >= col || (nr == r && nc == c) {
                continue;
            }
            let neighbor = board[nr as usize][nc as usize] & 1;
            if neighbor == 1 {
                count += 1;
            }
        }
    }
    count
}

fn can_live(val: i32, neighbor_count: i32) -> i32 {
    if neighbor_count == 2 {
        val
    } else if neighbor_count == 3 {
        1
    } else {
        0
    }
}
