impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        search(&matrix, 0, matrix.len() as i32 - 1, 0, matrix[0].len() as i32 - 1, target)
    }
}
// we can use master theorem to prove it is linear, but it is not straigt forward
fn search(matrix: &Vec<Vec<i32>>, sr: i32, er: i32, sc: i32, ec: i32 ,target: i32) -> bool {
    if sr > er || sc > ec {
        return false;
    }
    let r = (sr + (er - sr) / 2) as usize;
    let c = (sc + (ec - sc) / 2) as usize;
    if matrix[r][c] == target {
        return true;
    }
    if matrix[r][c] < target {
        search(matrix, r as i32 + 1, er, sc, ec, target) || search(matrix, sr, er, c as i32 + 1, ec, target)
    } else {
        search(matrix, sr, r as i32 - 1, sc, ec, target) || search(matrix, sr, er, sc, c as i32 - 1, target)
    }
}
// sr, er, sc ,ec
// if mid > target, search


/*
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut r = 0;
        let mut c = matrix[0].len() - 1;
        while r < matrix.len() {
            if matrix[r][c] == target {
                return true;
            }
            if matrix[r][c] < target {
                r += 1;
            } else {
                if c == 0 {
                    break;
                }
                c -= 1;
            }
        }
        false
    }
}

// we start at the top right, it is monotonous
// all numbers left to it is less and all numbers below are greater
*/
