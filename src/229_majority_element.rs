use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = HashMap::new();
        for n in &nums {
            if let Some(cnt) = ret.get_mut(n) {
                *cnt += 1;
            } else if ret.len() < 2 {
                ret.insert(*n, 1);
            } else {
                for cnt in ret.values_mut() {
                    *cnt -= 1;
                }
                ret.retain(|_, v| *v != 0);
            }
        }
        for cnt in ret.values_mut() { *cnt = 0; }
        for n in &nums {
            if let Some(cnt) = ret.get_mut(n) { *cnt += 1; }
        }
        ret.into_iter().filter_map(|(n, cnt)| {
            if cnt > nums.len() / 3 { Some(n) } else { None }
        }).collect()
    }
}
