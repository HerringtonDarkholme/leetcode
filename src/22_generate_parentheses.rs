impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut dp = vec![vec!["".to_owned()]];
        get_p(n as usize, &mut dp)
    }
}

fn get_p(n: i32, dp: &mut Vec<Vec<String>>) -> Vec<String> {
    if n < dp.len() {
        return dp[n].clone()
    }
    let mut r = vec![];
    for i in (0..n).rev() {
        for a in get_p(i, dp) {
            for b in get_p(n - i - 1, dp) {
                r.push(format!("({}){}", a, b));
            }
        }
    }
    dp.push(r);
    return dp[n].clone()
}
