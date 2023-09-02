impl Solution {
    pub fn min_extra_char(s: String, dict: Vec<String>) -> i32 {
        let n = s.len();

        let mut d = vec![0; n + 1]; // s[..i]
        for i in 1..=n {
            d[i] = d[i - 1] + 1;
            let s = &s[..i];
            for w in &dict {
                if s.ends_with(w) {
                    d[i] = d[i].min(d[i - w.len()]);
                }
            }
        }

        *d.last().unwrap()
    }
}
/*

use std::collections::HashSet;
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let dict: HashSet<_> = dictionary.into_iter().collect();
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for j in 0..s.len() {
            for i in (0..=j).rev() {
                if dict.contains(&s[i..=j]) {
                    continue;
                }
                if i == j {
                    dp[i][j] = 1;
                    continue;
                }
                let mut min = i32::MAX;
                for k in i..j {
                    min = min.min(dp[i][k] + dp[k+1][j]);
                }
                dp[i][j] = min;
            }
        }
        dp[0][s.len() - 1]
    }
}

// cost(i, j) = 
//  1 if s[i..=j] in dict
//  else min(cost(i, k) + cost(k+1, j) for k in i..=j)
*/
