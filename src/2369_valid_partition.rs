impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        if nums.len() == 2 {
            return nums[0] == nums[1];
        }
        let mut valid = vec![false; nums.len()];
        valid[1] = nums[0] == nums[1];
        valid[2] = valid[1] && nums[2] == nums[1] || 
            nums[0] + 1 == nums[1] && nums[1] + 1 == nums[2];
        for n in 3..nums.len() {
            valid[n] = 
                nums[n] == nums[n - 1] && valid[n - 2] ||
                nums[n] == nums[n - 1] && nums[n] == nums[n - 2] && valid[n - 3] ||
                nums[n] == nums[n - 1] + 1 && nums[n - 1] == nums[n - 2] + 1 && valid[n - 3];
        }
        valid[valid.len() - 1]
    }
}
//  F T T F T F T
// [4 4 4 5 6 1 1 1]
// valid(n) = 
//   nums[n] == nums[n - 1] && valid(n - 2) // if take two 
//   nums[n] == nums[n - 1] && nums[n] == nums[n - 2] && valid(n - 3) // take three
//   nums[n] == nums[n - 1] - 1 && nums[n - 1] == nums[n - 2] && valid(n - 3) 
