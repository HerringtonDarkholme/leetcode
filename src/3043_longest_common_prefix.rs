use std::collections::HashSet;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for mut num in arr1 {
            while num != 0 {
                set.insert(num);
                num /= 10;
            }
        }        
        let mut max = 0;
        for mut num in arr2 {
            while num != 0 {
                if set.contains(&num) {
                    max = max.max(num.ilog10() + 1);
                    break;
                }
                num /= 10;
            }
        }
        max as i32
    }
}
