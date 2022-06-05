impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut placed = vec![];
        let mut ret = 0;
        n_queen(n, &mut placed, &mut ret);
        ret
    }
}

fn n_queen(n: i32, placed: &mut Vec<(i32, i32)>, ret: &mut i32) {
    if placed.len() == n as usize {
        *ret += 1;
        return;
    }
    let r = placed.len() as i32;
    for c in 0..n {
        if conflict(placed, r, c) {
            continue;
        }
        placed.push((r, c));
        n_queen(n, placed, ret);
        placed.pop();
    }
}

fn conflict(placed: &Vec<(i32, i32)>, r: i32, c: i32) -> bool {
    for &(x, y) in placed {
        if c == y || (r - c) == (x - y) || (r + c) == (x + y) {
            return true;
        }
    }
    false
}
