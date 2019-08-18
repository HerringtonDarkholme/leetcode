pub struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        Solution::quick_select(0, nums.len() - 1, &mut nums, k as usize - 1)

    }
    fn quick_select(start: usize, end: usize, nums: &mut [i32], k: usize) -> i32 {
        let pivot = nums[end];
        let mut j = start;
        for i in start..end {
            if nums[i] >= pivot {
                nums.swap(i, j);
                j += 1;
            }
        }
        nums.swap(j, end);
        if j == k {
            pivot
        } else if j > k {
            Solution::quick_select(start, j - 1, nums, k)
        } else {
            Solution::quick_select(j + 1, end, nums, k)
        }
    }
}
