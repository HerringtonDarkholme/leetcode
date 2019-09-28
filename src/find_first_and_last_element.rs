pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1]
        }
        vec![find_min(&nums, target), find_max(&nums, target)]
    }
}

fn find_min(nums: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        let t = nums[mid];
        if mid >= nums.len() - 1 {
            return if t == target { mid as i32 } else { -1 }
        }
        let t1 = nums[mid + 1];
        if t < target && t1 == target {
            return (mid + 1) as i32
        } else if t < target && t1 > target {
            return -1
        } else if t >= target {
            if mid == 0 {
                return if t == target { 0 } else { - 1};
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    -1
}

fn find_max(nums: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        let t = nums[mid];
        if mid >= nums.len() - 1 {
            return if t == target { mid as i32 } else { -1 }
        }
        let t1 = nums[mid + 1];
        if t == target && t1 > target {
            return mid as i32
        } else if t < target && t1 > target {
            return -1
        } else if t1 <= target {
            low = mid + 1;
        } else {
            if mid == 0 {
                return if t == target { 0 } else { - 1};
            }
            high = mid - 1;
        }
    }
    -1
}
