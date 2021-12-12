impl Solution {
    pub fn sub_array_ranges(mut nums: Vec<i32>) -> i64 {
        let mut ret = 0i64;
        for i in 0..nums.len() {
            let mut min = nums[i];
            let mut max = nums[i];
            for j in i..nums.len() {
                min = min.min(nums[j]);
                max = max.max(nums[j]);
                ret += (max - min) as i64;
            }
        }
        ret
    }
}
