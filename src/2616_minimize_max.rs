impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];
        while left < right {
            let mid = left + (right - left) / 2;
            if count_pairs(&nums, mid) < p {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left 
    }
}

fn count_pairs(nums: &[i32], threshold: i32) -> i32 {
    let mut index = 0;
    let mut count = 0;
    while index < nums.len() - 1 {
        if nums[index + 1] - nums[index] <= threshold {
            count += 1;
            index += 1;
        }
        index += 1;
    }
    count
}
