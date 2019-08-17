pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut m = 0;
        for i in nums {
            if count == 0 {
                m = i;
                count = 1;
            } else if i == m {
                count += 1;
            } else {
                count -= 1;
            }
        }
        m
    }
}
