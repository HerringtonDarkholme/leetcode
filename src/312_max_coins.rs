impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let start = 1;
        let end = nums.len();
        nums.push(1);
        nums.insert(0, 1);
        let mut dp = vec![vec![-1; nums.len()]; nums.len()];
        max_coins(&nums, start, end, &mut dp)
    }
}

// consider the last ballon to burst,
// total_point = last_shot + prev_ballons_coins + next_ballons_coins
// this breaks ballons to two sets, and the value is delimited by start/end
fn max_coins(nums: &[i32], start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[start][end] != -1 {
        return dp[start][end];
    }
    let mut max = 0;
    let prev = nums[start - 1];
    let next = nums[end + 1];
    // traverse all ballons, find the last shot with max points
    for i in start..=end {
        // start < end is ok, since it fallbacks to 0
        let p = max_coins(nums, start, i - 1, dp);
        let n = max_coins(nums, i + 1, end, dp);
        let last_shot = prev * nums[i] * next;
        max = max.max(last_shot + p + n);
    }
    dp[start][end] = max;
    max
}
                      

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
