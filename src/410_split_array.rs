impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let m = m as usize;
        let mut dp = vec![vec![-1; m + 1]; nums.len()];
        split(&nums, 0, m, &mut dp)
    }
}

fn split(nums: &[i32], s: usize, m: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[s][m] >= 0 {
        return dp[s][m];
    }
    if m == 1 {
        dp[s][1] = nums[s..].iter().sum();
        return dp[s][1];
    }
    let mut max = i32::MAX;
    let mut sum = 0;
    for i in s..nums.len() - m + 1 {
        sum += nums[i];
        max = split(nums, i + 1, m - 1, dp).max(sum).min(max);
    }
    dp[s][m] = max;
    max
}
// dp[s][m] =x
