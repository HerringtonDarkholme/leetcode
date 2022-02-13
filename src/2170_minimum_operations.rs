use std::collections::HashMap;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0
        }
        let len = nums.len();
        let mut even = HashMap::new();
        let mut odd = HashMap::new();
        for i in 0..nums.len() {
            if i % 2 == 0 {
                *even.entry(nums[i]).or_insert(0) += 1;
            } else {
                *odd.entry(nums[i]).or_insert(0) += 1;
            }
        }
        let mut even: Vec<_> = even.into_iter().map(|(k, v)| (v, k)).collect();
        even.sort();
        let mut odd: Vec<_> = odd.into_iter().map(|(k, v)| (v, k)).collect();
        odd.sort();
        let max_even = even.last().unwrap();
        let max_odd = odd.last().unwrap();
        let next_even = even.get(even.len() - 2).unwrap_or(&(0, 0)).0;
        let next_odd = odd.get(odd.len() - 2).unwrap_or(&(0, 0)).0;
        if max_even.1 != max_odd.1 {
            (len - max_even.0 - max_odd.0) as i32
        } else if max_even.0 > max_odd.0 {
            (len - max_even.0 - next_odd) as i32
        } else if max_odd.0 > max_even.0 {
            (len - max_odd.0 - next_even) as i32
        } else {
            (len - max_even.0 - next_even.max(next_odd)) as i32
        }
    }
}

