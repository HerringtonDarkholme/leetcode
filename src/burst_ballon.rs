// leetcode 312
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.push(1);
        nums.insert(0, 1);
        let mut dp = vec![vec![-1; nums.len()]; nums.len()];
        dp_from_last_burst(&nums, 0, nums.len() - 1, &mut dp)
    }
}

// consider the last ballon to burst, because last burst ballon can divide
// ballons to two groups, one starts with the ballon and one ends with the ballon
fn dp_from_last_burst(nums: &[i32], start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if start + 1 == end {
        return 0
    }
    if dp[start][end] >= 0 {
        return dp[start][end]
    }
    let mut max = 0;
    for i in start+1..end {
        // if i is the last ballon to burst, its point should be
        // the start * i * end
        let point = nums[start] * nums[i] * nums[end] +
        dp_from_last_burst(nums, start, i, dp) +
        dp_from_last_burst(nums, i, end, dp);
        max = max.max(point);
    }
    dp[start][end] = max;
    max
}
