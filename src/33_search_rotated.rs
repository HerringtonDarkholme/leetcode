pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let e = nums[mid];
            if e == target {
                return mid as i32
            }
            let search_right = 
                if e > nums[right] { // peak on right
                    e < target || nums[right] >= target
                } else { // peak on left
                    e < target && nums[right] >= target
                };
            if search_right {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    }
}


/*
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1
        }
        search_aux(&nums, target, 0, nums.len() - 1)
    }
}

fn search_aux(nums: &Vec<i32>, target: i32, mut low: usize, mut high: usize) -> i32 {
    if low == high {
        return if nums[low] == target { low as i32 } else { -1 }
    }
    let mid = low + (high - low) / 2;
    let e = nums[mid];
    if e == target {
        return mid as i32
    }
    let is_continuous = nums[low] < nums[high];
    if is_continuous {
        return if e < target {
            search_aux(nums, target, mid + 1, high)
        } else {
            search_aux(nums, target, low, mid)
        }
    }
    let pre = search_aux(nums, target, low, mid);
    if pre != -1 {
        pre
    } else {
        search_aux(nums, target, mid + 1, high)
    }
}
*/
