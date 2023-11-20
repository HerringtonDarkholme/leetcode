impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for i in 0..k as usize {
            sum += nums[i];
        }
        let mut max = sum;
        for i in (k as usize)..nums.len() {
            sum += nums[i];
            sum -= nums[i - k as usize];
            max = max.max(sum);
        }
        max as f64 / k as f64
    }
}
