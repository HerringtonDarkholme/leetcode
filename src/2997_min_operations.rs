impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        for num in nums {
            ret ^= num;
        }
        (k ^ ret).count_ones() as i32
    }
}
