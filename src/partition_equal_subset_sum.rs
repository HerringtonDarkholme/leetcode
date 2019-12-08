pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 != 0 {
            return false
        }
        let mut dp = vec![false; sum / 2 + 1];
        dp[0] = true;
        for n in nums {
            let mut next = dp.clone();
            for (d, &b) in dp.iter().enumerate() {
                if (!b) {
                    continue;
                }
                let idx = d + n as usize;
                if idx <= sum / 2 {
                    next[idx] = true;
                }
            }
            dp = next;
        }
        dp[sum / 2]
    }
}
