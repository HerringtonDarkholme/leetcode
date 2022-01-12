impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        // we have a sliding window of length n, n == 1's count
        let mut n = nums.iter().filter(|&&num| num == 1).count();
        // ciruclar property
        nums.extend(nums.clone());
        // how many swaps do we need in the initial sliding window?
        let mut swaps = nums[..n].iter().filter(|&&num| num == 0).count() as i32;
        let mut min = swaps;
        for i in n..nums.len() {
            // update swaps
            swaps += nums[i-n]-nums[i];
            min = min.min(swaps);
        }
        min
    }
}
