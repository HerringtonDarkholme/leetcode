impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        let mut dp = vec![0; n2 + 1];
        let mut dp_prev = vec![0; n2 + 1];

        for i in 1..=n1 {
            for j in 1..=n2 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[j] = 1 + dp_prev[j - 1];
                } else {
                    dp[j] = dp[j - 1].max(dp_prev[j]);
                }
            }
            dp_prev = dp.clone();
        }

        dp[n2]
    }
}
