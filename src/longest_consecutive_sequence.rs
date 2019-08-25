pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums.into_iter().collect();
        set.iter().fold(0, |max, &n| if !set.contains(&(n-1)) {
            let longest = (n..).skip_while(|i| set.contains(&i)).next().unwrap();
            max.max(longest - n)
        } else {
            max
        })
    }
}

/*
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut starts = HashMap::new();
        let mut ends = HashMap::new();
        let mut seen = HashSet::new();
        let mut max = 0;
        for i in nums {
            if seen.contains(&i) {
                continue;
            }
            seen.insert(i);
            let prev = i - 1;
            let next = i + 1;
            let new_end = *starts.get(&next).unwrap_or(&i);
            let new_start = *ends.get(&prev).unwrap_or(&i);
            max = max.max(new_end - new_start + 1);
            ends.insert(new_end, new_start);
            starts.insert(new_start, new_end);
        }
        max
    }
}
*/
