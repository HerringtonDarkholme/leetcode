impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut dp = vec![vec![0, 1]];
        gray_code(n as usize - 1, &mut dp)
    }
}

fn gray_code(n: usize, dp: &mut Vec<Vec<i32>>) -> Vec<i32> {
    if dp.len() > n {
        return dp[n].clone()
    }
    let mut n1 = gray_code(n - 1, dp);
    let mut n2: Vec<_> = dp[n - 1]
        .iter()
        .map(|e| e | 1 << n)
        .collect();
    n2.reverse();
    n1.extend(n2);
    dp.push(n1);
    dp[n].clone()
}
