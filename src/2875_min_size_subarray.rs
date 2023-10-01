impl Solution {
    pub fn min_size_subarray(mut nums: Vec<i32>, target: i32) -> i32 {
        let target = target as i64;
        let sum: i64 = nums.iter().map(|&n| n as i64).sum();
        let rept = (target / sum) * nums.len() as i64;
        let remain = target % sum;
        let mut min = -1;
        let mut s = 0;
        let mut left = 0;
        nums.extend(nums.clone());
        for right in 0..nums.len() {
            s += nums[right] as i64;
            while s > remain && left <= right {
                s -= nums[left] as i64;
                left += 1;
            }
            if s == remain {
                if min < 0 {
                    min = (right - left + 1) as i32;
                } else {
                    min = min.min((right - left + 1) as i32);
                }
            }
        }
        if min >= 0 {
            rept as i32 + min
        } else {
            -1
        }
    }
}
// num_of_array = target / sum(nums) 
// remainin = target % sum(nums) = tail + head
