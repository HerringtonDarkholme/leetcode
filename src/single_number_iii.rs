pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = 0;
        for &n in nums.iter() {
            acc ^= n;
        }
        // two targets must be within two different groups
        // so the problem becomes find one single number
        acc = (acc & -acc);
        let mut a = 0;
        let mut b = 0;
        for &n in nums.iter() {
            if acc & n != 0 {
                a ^= n;
            } else {
                b ^= n;
            }
        }
        vec![a, b]
    }
}
