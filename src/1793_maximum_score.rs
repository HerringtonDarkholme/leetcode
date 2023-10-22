impl Solution {
    pub fn maximum_score(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut i = k as usize;
        let mut j = k as usize;
        let mut curr_min = nums[i];
        let mut ret = nums[i] as usize;
        while i > 0 || j < nums.len() - 1 {
            if i == 0 {
                j += 1;
            } else if j == nums.len() - 1 {
                i -= 1;
            } else if nums[i - 1] > nums[j + 1] {
                i -= 1;
            } else {
                j += 1;
            }
            curr_min = curr_min.min(nums[i]).min(nums[j]);
            ret = ret.max(curr_min as usize * (j - i + 1));
        }
        ret as i32
    }
}
