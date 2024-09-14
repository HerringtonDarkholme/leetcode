impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ret = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] < max { 
                i += 1;
                continue; 
            }
            let last_max = max;
            max = nums[i];
            let mut len = 0;
            while i < nums.len() && nums[i] == max {
                len += 1;
                i += 1;
            }
            if last_max == max {
                ret = ret.max(len);
            } else {
                ret = len;
            }
        }
        ret
    }
}
