impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<_> = nums.into_iter().enumerate().collect();
        nums.sort_unstable_by_key(|p| p.1);
        let mut prev = 0;
        let mut ret = vec![0; nums.len()];
        for i in 0..nums.len() {
            // same number
            if nums[i].1 == nums[prev].1 {
                ret[nums[i].0] = prev as i32;
            } else {
                ret[nums[i].0] = i as i32;
                prev = i;
            }
        }
        ret
    }
}
