impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut dp = vec![false; nums.len()];
        let mut max = 0;
        for i in 0..nums.len() {
            max = max.max(dfs(i, &nums, &mut dp, 0));
        }
        max
    }
}

fn dfs(pos: usize, nums: &[i32], dp: &mut Vec<bool>, depth: i32) -> i32 {
    if dp[pos] {
        return depth;
    }
    dp[pos] = true;
    return dfs(nums[pos] as usize, nums, dp, depth + 1);
}
//     | 
// 0 1 2 3 4 5 
