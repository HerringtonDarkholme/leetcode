impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut bitwise = 0;
        for &n in &nums {
            bitwise |= n;
        }
        helper(0, bitwise, &nums)
    }
}

fn helper(prev: i32, target: i32, nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return if prev == target { 1 } else { 0 };
    }
    helper(prev, target, &nums[1..]) + helper(prev | nums[0], target, &nums[1..])
}
