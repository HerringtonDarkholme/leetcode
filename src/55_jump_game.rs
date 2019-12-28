pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut m = 0;
        while m < nums.len() - 1 {
            let mut max = m;
            let mut next = m;
            for i in 1..=nums[m] as usize {
                let n = (m + i).min(nums.len() - 1);
                if n + nums[n] as usize >= max {
                    max = n + nums[n] as usize;
                    next = n;
                }
            }
            if next <= m {
                break;
            }
            m = next;
        }
        m == nums.len() - 1
    }
}
