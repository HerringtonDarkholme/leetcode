impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        validate(&board, |i, j| (i, j)) && 
        validate(&board, |i, j| (j, i)) && 
        validate(&board, |i, j| ((i / 3) * 3 + (j / 3), (i % 3) * 3 + (j % 3)))
    }
}

fn validate(board: &Vec<Vec<char>>, f: fn(usize, usize) -> (usize, usize)) -> bool {
    for i in 0..9 {
        let mut occ = [0; 9];
        for j in 0..9 {
            let (i, j) = f(i, j);
            let c = board[i][j] as u8 - '1' as u8;
            if c >= 9 { continue; }
            occ[c as usize] += 1;
        }
        for j in 0..9 {
            if occ[j] > 1 {
                return false
            }
        }
    }
    true
}
