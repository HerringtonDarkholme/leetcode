impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0 }
        let mut i = 0;
        let mut prod = 1;
        let mut ret = 0;
        for j in 0..nums.len() {
            prod *= nums[j];
            while prod >= k {
                prod /= nums[i];
                i += 1;
            }
            ret += j - i + 1;
        }
        ret as i32
    }
}
