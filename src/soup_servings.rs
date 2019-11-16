pub struct Solution;

use std::collections::HashMap;
// Not fun
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 3500 {
            return 1.0
        }
        let mut dp = HashMap::new();
        let n = (n + 24) / 25;
        dp.insert((0, 0), 0.5);
        for i in 1..=n {
            let mut j = 0;
            let mut p = 0.0;
            while p != 1.0 && j <= n {
                dp.insert((i, j), p);
                j += 1;
                p = 0.0;
                p += *dp.get(&((i-4).max(0), j)).unwrap_or(&1.0);
                p += *dp.get(&((i-3).max(0), (j-1).max(0))).unwrap_or(&1.0);
                p += *dp.get(&((i-2).max(0), (j-2).max(0))).unwrap_or(&1.0);
                p += *dp.get(&((i-1).max(0), (j-3).max(0))).unwrap_or(&1.0);
                p *= 0.25;
            }
        }
        dp[&(n, n)]
    }
}

/*
stack overflow LOL
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let mut dp = HashMap::new();
        f(n, n, &mut dp)
    }
}

fn f(a: i32, b: i32, dp: &mut HashMap<(i32, i32), f64>) -> f64 {
    if a <= 0 {
        return if b <= 0 { 0.5 } else { 1.0 }
    } else if b <= 0 {
        return 0.0
    }
    if let Some(ret) = dp.get(&(a, b)) {
        return *ret
    }
    let prob = 0.25 * (
        f(a - 100, b,      dp) +
        f(a - 75,  b - 25, dp) +
        f(a - 50,  b - 50, dp) +
        f(a - 25,  b - 75, dp)
    );
    dp.insert((a, b), prob);
    prob
}
*/
