impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
        let mut max = 0;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let d = if i == 0 || j == 0 {
                    0
                } else {
                    dp[i-1][j-1]
                };
                dp[i][j] = if nums1[i] == nums2[j] { d + 1 } else { 0 };
                max = max.max(dp[i][j]);
            }
        }
        max
    }
}
// dp[i][j] = the length of common subseq ends at i-th nums1 and j-th nums2
// dp[i][j] = if nums1[i] == nums2[j] { dp[i-1][j-1] + 1 } else { 0 }
