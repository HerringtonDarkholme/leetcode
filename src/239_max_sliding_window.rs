use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut stack = VecDeque::with_capacity(k);
        for i in 0..k {
            let num = nums[i];
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= num {
                stack.pop_back();
            }
            stack.push_back(i);
        }
        let mut ret = vec![nums[stack[0]]];
        for i in k..nums.len() {
            // remove element outside of sliding window
            if i - k >= stack[0] {
                stack.pop_front();
            }
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[i] {
                stack.pop_back();
            }
            stack.push_back(i);
            ret.push(nums[stack[0]]);
        }
        ret
    }
}

/*
use std::collections::BTreeMap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut map = BTreeMap::new();
        for i in 0..k {
            // insert in reverse order to get the max in btreemap
            *map.entry(-nums[i]).or_insert(0) += 1;
        }
        let mut ret = vec![
            // invert sign to get original value
            -*map.keys().next().unwrap()
        ];
        for i in k..nums.len() {
            // remove
            *map.get_mut(&-nums[i - k]).unwrap() -= 1;
            if map[&-nums[i - k]] == 0 {
                map.remove(&-nums[i - k]);
            }
            *map.entry(-nums[i]).or_insert(0) += 1;
            ret.push(
                -*map.keys().next().unwrap()
            );
        }
        ret
    }
}
*/
