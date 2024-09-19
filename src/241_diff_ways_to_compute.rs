impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let expr = split(expression);
        let len = expr.len();
        let mut dp = vec![vec![vec![]; len]; len];
        let ret = compute(&expr, 0, len - 1, &mut dp);
        ret.into_iter().collect()
    }
}

fn split(expression: String) -> Vec<String> {
    let mut ret = vec![];
    let mut acc = String::new();
    for c in expression.chars() {
        if c == '+' || c == '-' || c == '*' {
            ret.push(acc);
            acc = String::new();
            ret.push(c.to_string());
        } else {
            acc.push(c);
        }
    }
    ret.push(acc);
    ret
}

fn compute(expr: &[String], s: usize, e: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    if !dp[s][e].is_empty() {
        return dp[s][e].clone();
    }
    if s == e {
        let ret: i32 = expr[s].parse().unwrap();
        dp[s][e].push(ret);
        return dp[s][e].clone();
    }
    let mut i = s;
    while i + 2 <= e {
        let op1 = compute(expr, s, i, dp);
        let op2 = compute(expr, i + 2, e, dp);
        for n1 in op1 {
            for &n2 in &op2 {
                if &expr[i + 1] == "+"{
                    dp[s][e].push(n1 + n2);
                } else if &expr[i + 1] == "-" {
                    dp[s][e].push(n1 - n2);
                } else {
                    dp[s][e].push(n1 * n2);
                }
            }
        }
        i += 2;
    }
    return dp[s][e].clone();
}
