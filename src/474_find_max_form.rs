impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let l = strs.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];;
        
        for s in strs {
            let mut new_dp = vec![vec![0; m + 1]; n + 1];
            for j in 0..=n {
                for k in 0..=m {
                    let max_without = dp[j][k];
                    let mut max_with = 0;
                    let c = s.len();
                    let zc = s.chars().filter(|&a| a == '0').count();
                    let oc = c - zc;
                    if zc <= k && oc <= j {
                        max_with = 1 + dp[j - oc][k - zc];
                    }
                    new_dp[j][k] = max_without.max(max_with);
                }
            }
            dp = new_dp;
        }
        dp[n][m]
    }
}
