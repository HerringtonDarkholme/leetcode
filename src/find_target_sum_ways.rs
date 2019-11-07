pub struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as usize;
        if (sum as i32) + 1 -s <= 0 {
            return 0
        }
        let mut dp = vec![0; sum*2+1];
        dp[sum] = 1;
        for n in nums {
            let mut new_dp = vec![0; sum*2+1];
            for i in 0..dp.len() {
                if dp[i] == 0 {
                    continue;
                }
                new_dp[i + n as usize] += dp[i];
                new_dp[i - n as usize] += dp[i];
            }
            dp = new_dp;
        }
        dp[s as usize + sum]
    }
}
