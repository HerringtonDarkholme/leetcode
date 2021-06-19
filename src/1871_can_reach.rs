impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        can_reach(s, min_jump as usize, max_jump as usize)
    }
}

fn can_reach(s: String, min: usize, max: usize) -> bool {
    let mut dp = vec![false; s.len()];
    dp[0] = true;
    let mut pre = 0;
    for (i, c) in s.chars().enumerate().skip(1) {
        if i >= min && dp[i - min] {
            pre += 1;
        }
        if i > max && dp[i - max - 1] {
            pre -= 1;
        }
        dp[i] = pre > 0 && c == '0';
    }
    dp[dp.len() - 1]
}
