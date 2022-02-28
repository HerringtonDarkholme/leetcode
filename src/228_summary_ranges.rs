impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut i = 0;
        let mut ret = vec![];
        for j in 1..=nums.len() {
            if j < nums.len() && nums[j] == nums[j - 1] + 1 {
                continue;
            }
            let s = if j - 1 == i {
                format!("{}", nums[i])
            } else {
                format!("{}->{}", nums[i], nums[j-1])
            };
            ret.push(s);
            i = j;
        }
        ret
    }
}
