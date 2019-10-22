pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1
        }
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            let e = nums[mid];
            if e == target {
                return mid as i32
            }
            let search_second = if e > nums[high] {
                e <= target || target <= nums[high]
            } else {
                e <= target && target <= nums[high]
            };
            if search_second {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if nums[low] == target {
            low as i32
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
