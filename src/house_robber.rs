pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut take = 0;
        let mut no_take = 0;
        for i in nums {
            let last_take = take;
            take = i + no_take;
            no_take = no_take.max(last_take);
        }
        no_take.max(take)
    }
}
