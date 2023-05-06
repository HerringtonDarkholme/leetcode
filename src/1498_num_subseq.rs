const M: i64 = 1_000_000_007;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort(); 
        let n = nums.len();
        let mut ans = 0;
        let mut left = 0; 
        let mut right = n - 1;
        let mut pow = vec![1; n]; // precompute powers of 2
        for i in 1..n {
            pow[i] = (pow[i - 1] * 2) % M;
        }
        while left <= right {
            if nums[left] + nums[right] <= target { 
                ans = (ans + pow[right - left]) % M; 
                left += 1; 
            } else if right > 0 {
                right -= 1; 
            } else {
                break
            }
        }
        ans as i32
    }
}
