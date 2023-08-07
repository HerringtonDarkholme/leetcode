impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let word: Vec<_> = word.chars().collect();
        for r in 0..board.len() as i32 {
            for c in 0..board[0].len() as i32 {
                if exist(r, c, &board, &mut visited, &word) {
                    return true;
                }
            }
        }
        false
    }
}
fn exist(
    r: i32, 
    c: i32,
    board: &Vec<Vec<char>>, 
    visited: &mut Vec<Vec<bool>>, 
    word: &[char]
) -> bool {
    if word.is_empty() {
        return true;
    }
    if r < 0 || r >= board.len() as i32 || c < 0 || c >= board[0].len() as i32 {
        return false;
    }
    if board[r as usize][c as usize] != word[0] {
        return false;
    }
    if visited[r as usize][c as usize] {
        return false;
    }
    visited[r as usize][c as usize] = true;
    for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if exist(r + dx, c + dy, board, visited, &word[1..]) {
            return true;
        }
    }
    visited[r as usize][c as usize] = false;
    false
}
