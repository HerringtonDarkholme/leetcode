impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut start, mut min, mut max) = (0, 0, 0);
        let mut ret = 0;
        for (i, &n) in nums.iter().enumerate() {
            if n < min_k || n > max_k { // OOB, reset all counter
                (min, max, start) = (i + 1, i + 1, i + 1);
                continue;
            }
            if n == min_k { min = i; }
            if n == max_k { max = i; }
            if min_k == nums[min] && max_k == nums[max] { // bound all set
                ret += (max.min(min) - start + 1); // count subs
            }
        }
        ret as i64
    }
}
             
// S     M   Em
// 4 1 3 5 2 1
