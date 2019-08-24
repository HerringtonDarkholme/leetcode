pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let mut range: Vec<_> = (1..=max_choosable_integer).collect();
        let mut hash = HashMap::new();
        if desired_total == 0 {
            return true
        }
        if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false
        }
        Solution::can_win(range, 0, desired_total, &mut hash)
    }
    fn can_win(range: Vec<i32>, current: i32, target: i32, hash: &mut HashMap<String, bool>) -> bool {
        let key = Solution::make_hash(&range);
        if let Some(&ret) = hash.get(&key) {
            return ret
        }
        let ret = range.iter().enumerate().any(|(i, &v)| {
            if current + v >= target {
                return true
            }
            let mut r = range.clone();
            r.remove(i);
            Solution::can_win(r, current + v, target, hash) == false
        });
        hash.insert(key, ret);
        ret
    }
    fn make_hash(range: &Vec<i32>) -> String {
        let v: Vec<_> = range.iter().map(i32::to_string).collect();
        v.join("_")
    }
}
