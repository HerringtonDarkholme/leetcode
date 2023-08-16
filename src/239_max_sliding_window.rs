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
