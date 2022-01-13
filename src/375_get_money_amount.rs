impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n+1]; n+1];
        money(1, n, &mut dp) as i32
    }
}

fn money(s: usize, e: usize, dp: &mut Vec<Vec<u16>>) -> u16 {
    if s >= e {
        return 0;
    }
    if dp[s][e] != 0 {
        return dp[s][e];
    }

    let mut min = u16::MAX;
    for i in s..e {
        let small = money(s, i - 1, dp);
        let large = money(i + 1, e, dp);
        min = min.min(small.max(large) + i as u16);
    }
    dp[s][e] = min;
    min
}
