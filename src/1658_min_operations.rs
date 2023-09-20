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
/*
use std::collections::HashMap;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, x: i32) -> i32 {
        nums.push(0);
        nums.insert(0, 0);
        let mut map = HashMap::new();
        let mut sum = 0;
        for (i, &n) in nums.iter().rev().enumerate() {
            sum += n;
            map.insert(sum, i);
        }
        sum = 0;
        let mut min = -1;
        for (i, &n) in nums.iter().enumerate() {
            sum += n;
            if let Some(j) = map.get(&(x - sum)) {
                if j + i > nums.len() - 2 {
                    continue;
                }
                min = if min < 0 { (i + j) as i32 } else {
                    min.min((i + j) as i32)
                };
            }
        }
        min
    }
}

*/
