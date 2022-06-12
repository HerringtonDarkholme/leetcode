impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut left = vec![0; nums.len() + 1];
        let mut right = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            left[i + 1] = left[i] + nums[i];
            right[i + 1] = right[i] + nums[nums.len() - 1 - i];
        }
        let mut sum = -1;
        for i in 0..left.len() {
            if x < left[i] {
                break;
            }
            if let Ok(j) = right.binary_search(&(x - left[i])) {
                if i + j > nums.len() {
                    break;
                }
                if sum < 0 {
                    sum = (i + j) as i32;
                } else {
                    sum = sum.min((i + j) as i32);
                }
            }
        }
        sum
    }
}
