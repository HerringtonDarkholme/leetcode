pub struct Solution;

impl Solution {
    pub fn makesquare(nums: Vec<i32>) -> bool {
        let len: i32 = nums.iter().sum();
        if len % 4 != 0 || len == 0 {
            return false
        }
        let side = len / 4;
        Solution::use_match(&nums, (side, side, side), side)
    }

    fn use_match(nums: &[i32], sides: (i32, i32, i32), side: i32) -> bool {
        // three edges enough cuz the sum is known
        let (n0, n1, n2) = sides;
        if n0 == 0 && n1 == 0 && n2 == 0 {
            return true
        }
        if nums.is_empty() {
            return false
        }
        let n = nums[0];
        let nums = &nums[1..];
        let m = Solution::use_match;
        n0 >= n && m(nums, (n0 - n, n1, n2), side) ||
        n0 != n1 && n1 >= n && m(nums, (n0, n1 - n, n2), side) ||
        n1 != n2 && n2 >= n && m(nums, (n0, n1, n2 - n), side) ||
        side != n2 && m(nums, (n0, n1, n2), side)
    }
}
