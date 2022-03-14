impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        max(&nums, k as usize)
    }
}

fn max(nums: &[i32], k: usize) -> i32 {
    if nums.len() == 1 {
        return if k % 2 == 0 { nums[0] } else { -1 }
    }
    let mut max = -1;
    for i in 0..nums.len() {
        if i > k { // k move cannot reach nums[i]
            break;
        }
        // either remove k elements or remove at most k-2 element and put back
        // i + 1 == k cannot be taken since nums[i] is either taken or occluded by element put back
        if i + 1 < k || i == k {
            max = max.max(nums[i]);
        }
    }
    max
}
