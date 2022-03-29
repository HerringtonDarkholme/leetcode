impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let nums: Vec<_> = nums.into_iter().map(|u| u as usize).collect();
        let mut slow = nums[nums[0]];
        let mut fast = nums[nums[nums[0]]];
        while slow != fast {
            slow = nums[slow];
            fast = nums[nums[fast]];
        }
        slow = nums[0];
        while slow != fast {
            slow = nums[slow];
            fast = nums[fast];
        }
        slow as i32
    }
}
