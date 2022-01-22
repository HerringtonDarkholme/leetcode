impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![None; n as usize + 1];
        dp[0] = Some(false);
        let squares = find_squares(n);
        run(n, &squares, &mut dp)
    }
}

fn find_squares(n: i32) -> Vec<i32> {
    let mut ret = vec![];
    let mut i = 1;
    while i * i <= n {
        ret.push(i * i);
        i += 1;
    }
    ret
}

fn run(n: i32, squares: &[i32], dp: &mut Vec<Option<bool>>) -> bool {
    if let Some(b) = &dp[n as usize] {
        return *b;
    }
    for &s in squares.iter() {
        if n < s {
            break;
        }
        if !run(n - s, squares, dp) {
            dp[n as usize] = Some(true);
            return true
        }
    }
    dp[n as usize] = Some(false);
    return false;
}
