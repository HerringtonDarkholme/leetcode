impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let l = nums.len();
        (nums[l - 1] * nums[l - 2]) - (nums[0] * nums[1])
    }
}
