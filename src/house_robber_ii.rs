pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }
        if nums.len() < 1 {
            return nums[0]
        }

        let mut take_f = nums[0];
        let mut no_take_f = i32::min_value(); // fake value, it means we cannot rob 2nd house
        let mut take = 0;
        let mut no_take = 0;
        for i in 1..nums.len() - 1 {
            let last_take = take;
            take = no_take + nums[i];
            no_take = no_take.max(last_take);
            let last_take_f = take_f;
            take_f = no_take_f + nums[i]; // 2nd house take_f will be i32::min_value
            no_take_f = no_take_f.max(last_take_f);
        }
        let len = nums.len();
        let last = nums[len - 1];
        let no_last = take_f.max(no_take_f);
        let has_last = take.max(no_take+last);
        no_last.max(has_last)
    }
}
