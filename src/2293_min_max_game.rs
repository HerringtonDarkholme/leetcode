impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut new_nums = vec![];
            for i in 0..nums.len() / 2 {
                new_nums.push(if i % 2 == 0 {
                    nums[2 * i].min(nums[2 * i + 1])
                } else {
                    nums[2 * i].max(nums[2 * i + 1])
                });
            }
            if nums.len() % 2 == 1 {
                new_nums.push(nums[nums.len() - 1]);
            }
            nums = new_nums;
        }
        nums[0]
    }
}
