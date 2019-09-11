pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut prod = vec![1; len];
        let mut p = 1;
        for i in 1..=len {
            let temp = nums[len - i];
            prod[len - i] = p;
            p *= temp;
        }
        p = 1;
        for i in 0..len {
            prod[i] = p * prod[i];
            p *= nums[i];
        }
        prod
    }
}
