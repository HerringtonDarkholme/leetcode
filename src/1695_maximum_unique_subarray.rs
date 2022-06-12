impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s = vec![0;n+1];
        for i in 0..n {
            s[i+1] = s[i] + nums[i];
        }
        let mut res = 0;
        let mut pos = std::collections::HashMap::new();
        let mut left = 0;
        for right in 0..n {
            if let Some(p) = pos.get_mut(&nums[right]){
                left = left.max(*p + 1);
                *p = right;
            } else {
                pos.insert(nums[right], right);
            }
            res = res.max(s[right+1] - s[left]);
        }
        res
    }
}
