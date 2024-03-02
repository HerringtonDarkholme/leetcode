impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums.len());
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let l = nums[left] * nums[left];
            let r = nums[right] * nums[right];
            if l >= r {
                ret.push(l);
                left += 1;
            } else {
                ret.push(r);
                right -= 1;
            }
        }
        ret.reverse();
        ret
    }
}
