use std::collections::HashMap;
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let t = to_map(target);
        let a = to_map(arr);
        t == a
    }
}

fn to_map(nums: Vec<i32>) -> HashMap<i32, usize> {
    let mut ret = HashMap::new();
    for num in nums {
        *ret.entry(num).or_insert(0) += 1;
    }
    ret
}
