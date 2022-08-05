impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let nums: Vec<_> = nums.into_iter().map(|n| n as usize).collect();
        let mut dp = vec![-1; 1001];
        sum(&nums, target as usize, &mut dp)
    }
}

fn sum(nums: &[usize], target: usize, dp: &mut Vec<i32>) -> i32 {
    if target == 0 {
        return 1;
    }
    if dp[target] >= 0 {
        return dp[target];
    }
    let mut ways = 0;
    for &n in nums {
        if n <= target {
            ways += sum(nums, target - n, dp);
        }
    }
    dp[target] = ways;
    ways
}
