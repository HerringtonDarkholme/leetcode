pub struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let l = nums.len();
        if l % 2 == 0 {
            return true
        }
        // dp is a 2-D vector. dp[s][e] means
        // a subslice starting from s, ending at e
        // how much the first player can win (minus second player scoe)
        let mut dp = vec![vec![0; l + 1]; l];
        let mut s = l;
        loop {
            s -= 1;
            let mut e = s + 1;
            while e < l {
                let take_first = nums[s] - dp[s + 1][e];
                let take_last = nums[e] - dp[s][e - 1];
                dp[s][e] = take_first.max(take_last);
                e += 1;
            }
            if s == 0 {
                break;
            }
        }
        dp[0][l - 1] >= 0
    }
}
