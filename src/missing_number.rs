pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut len = 0;
        for n in nums {
            ret ^= n;
            len += 1;
        }
        for i in 0..=len {
            ret ^= i;
        }
        ret
    }
}
